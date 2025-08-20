use tokenizers::models as tkm;
use tokenizers::models::TrainerWrapper;
use tokenizers::tokenizer::{
    DecoderWrapper, ModelWrapper, NormalizerWrapper, PostProcessorWrapper, PreTokenizerWrapper,
    TokenizerBuilder, TokenizerImpl,
};

use crate::buffer_utils::{get_call_message, set_call_result};
use crate::messages::decoders;
use crate::messages::decoders::params::Params as d_params;
use crate::messages::processors;
use crate::messages::processors::params::Params as p_params;
use crate::messages::trainers;
use crate::messages::trainers::trainer_params;
use crate::messages::{self, CallStatus, ConversionError};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tokenizer_from_train(
    instance_ptr: *mut *mut TokenizerImpl<
        ModelWrapper,
        NormalizerWrapper,
        PreTokenizerWrapper,
        PostProcessorWrapper,
        DecoderWrapper,
    >,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<trainers::TrainerParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    if params.trainer.is_none() {
        crate::set_empty_output!(out_ptr, out_len);
        return CallStatus::EmptyParams.into();
    }
    let mut trainer: TrainerWrapper = match_trainer(params.trainer.unwrap());
    let trained_model: ModelWrapper = match trainer {
        TrainerWrapper::BpeTrainer(_) => tkm::bpe::BPE::default().into(),
        TrainerWrapper::WordPieceTrainer(_) => tkm::wordpiece::WordPiece::default().into(),
        TrainerWrapper::WordLevelTrainer(_) => tkm::wordlevel::WordLevel::default().into(),
        TrainerWrapper::UnigramTrainer(_) => tkm::unigram::Unigram::default().into(),
    };
    let mut builder: TokenizerBuilder<
        ModelWrapper,
        NormalizerWrapper,
        PreTokenizerWrapper,
        PostProcessorWrapper,
        DecoderWrapper,
    > = TokenizerBuilder::new();
    builder = builder.with_model(trained_model);
    if let Some(normalizer) = params.normalizer {
        let n = unsafe { &*(normalizer as usize as *const NormalizerWrapper) };
        builder = builder.with_normalizer(Some(n.clone()));
    }
    if let Some(pre_tokenizer) = params.pre_tokenizer {
        let pt = unsafe { &*(pre_tokenizer as usize as *const PreTokenizerWrapper) };
        builder = builder.with_pre_tokenizer(Some(pt.clone()));
    }
    if let Some(processor) = params.processor {
        builder = match get_processor(processor) {
            Ok(pp) => builder.with_post_processor(pp),
            Err(e) => {
                match e.1 {
                    Some(e) => set_call_result(messages::Error { details: e }, out_ptr, out_len),
                    None => crate::set_empty_output!(out_ptr, out_len),
                };
                return e.0.into();
            }
        }
    }
    if let Some(decoder) = params.decoder {
        builder = match get_decoder(decoder) {
            Ok(d) => builder.with_decoder(d),
            Err(e) => {
                match e.1 {
                    Some(e) => set_call_result(messages::Error { details: e }, out_ptr, out_len),
                    None => crate::set_empty_output!(out_ptr, out_len),
                };
                return e.0.into();
            }
        }
    }
    if let Some(truncation) = params.truncation {
        builder = match truncation.try_into() {
            Ok(trunc) => builder.with_truncation(Some(trunc)),
            Err(e) => {
                match e.1 {
                    Some(e) => set_call_result(messages::Error { details: e }, out_ptr, out_len),
                    None => crate::set_empty_output!(out_ptr, out_len),
                };
                return e.0.into();
            }
        }
    }
    if let Some(padding) = params.padding {
        builder = match padding.try_into() {
            Ok(pad) => builder.with_padding(Some(pad)),
            Err(e) => {
                match e.1 {
                    Some(e) => set_call_result(messages::Error { details: e }, out_ptr, out_len),
                    None => crate::set_empty_output!(out_ptr, out_len),
                };
                return e.0.into();
            }
        }
    }
    let mut tk = match builder.build() {
        Ok(t) => t,
        Err(e) => {
            set_call_result(
                messages::Error {
                    details: e.to_string(),
                },
                out_ptr,
                out_len,
            );
            return CallStatus::TokenizerBuildErrorDetails.into();
        }
    };
    if let Err(e) = tk.train(&mut trainer, params.files.iter()) {
        set_call_result(
            messages::Error {
                details: e.to_string(),
            },
            out_ptr,
            out_len,
        );
        return CallStatus::TokenizerTrainingErrorDetails.into();
    }
    let mut pretty = false;
    if let Some(pretty_p) = params.pretty {
        pretty = pretty_p;
    }
    if let Err(e) = tk.save(params.save_path, pretty) {
        set_call_result(
            messages::Error {
                details: e.to_string(),
            },
            out_ptr,
            out_len,
        );
        return CallStatus::TokenizerSaveErrorDetails.into();
    }
    let tk_p = Box::new(tk);
    unsafe {
        *instance_ptr = Box::into_raw(tk_p);
    }
    CallStatus::Ok.into()
}

fn match_trainer(selected_trainer: trainer_params::Trainer) -> TrainerWrapper {
    match selected_trainer {
        trainer_params::Trainer::BpeTrainer(params) => params.into(),
        trainer_params::Trainer::WordPieceTrainer(params) => params.into(),
        trainer_params::Trainer::WordLevelTrainer(params) => params.into(),
        trainer_params::Trainer::UnigramTrainer(params) => params.into(),
    }
}

fn get_processor(
    processor: processors::ProcessorWrapperParams,
) -> Result<Option<PostProcessorWrapper>, ConversionError> {
    if processor.params.len() == 0 {
        return Ok(None);
    }
    let r: Result<Vec<PostProcessorWrapper>, ConversionError> = processor
        .params
        .into_iter()
        .filter_map(|params| params.params)
        .map(|params| match_processor(params))
        .collect();
    Ok(match r {
        Ok(mut v) => {
            if v.len() <= 1 {
                v.pop()
            } else {
                Some(tokenizers::processors::sequence::Sequence::new(v).into())
            }
        }
        Err(e) => return Err(e),
    })
}

fn match_processor(params: p_params) -> Result<PostProcessorWrapper, ConversionError> {
    Ok(match params {
        p_params::RobertaProcessing(params) => params.into(),
        p_params::BertProcessing(params) => params.into(),
        p_params::ByteLevel(params) => params.into(),
        p_params::TemplateProcessing(params) => params.try_into()?,
    })
}

fn get_decoder(
    decoder: decoders::DecoderWrapperParams,
) -> Result<Option<DecoderWrapper>, ConversionError> {
    if decoder.params.len() == 0 {
        return Ok(None);
    }
    let r: Result<Vec<DecoderWrapper>, ConversionError> = decoder
        .params
        .into_iter()
        .filter_map(|params| params.params)
        .map(|params| match_decoder(params))
        .collect();
    Ok(match r {
        Ok(mut v) => {
            if v.len() <= 1 {
                v.pop()
            } else {
                Some(tokenizers::decoders::sequence::Sequence::new(v).into())
            }
        }
        Err(e) => return Err(e),
    })
}

fn match_decoder(params: d_params) -> Result<DecoderWrapper, ConversionError> {
    Ok(match params {
        d_params::BpeDecoder(params) => params.into(),
        d_params::ByteLevel(params) => params.into(),
        d_params::WordPiece(params) => params.into(),
        d_params::Metaspace(params) => params.try_into()?,
        d_params::Ctc(params) => params.into(),
        d_params::Replace(params) => params.try_into()?,
        d_params::Fuse(params) => params.into(),
        d_params::Strip(params) => params.try_into()?,
        d_params::ByteFallback(params) => params.into(),
    })
}
