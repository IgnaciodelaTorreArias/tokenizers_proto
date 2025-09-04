use tokenizers::tokenizer::{
    DecoderWrapper, ModelWrapper, NormalizerWrapper, PostProcessorWrapper, PreTokenizerWrapper,
    TokenizerBuilder, TokenizerImpl,
};

use crate::buffer_utils::{get_call_message, set_call_result};
use crate::messages::{
    self, CallStatus, decoders::get_decoder, processors::get_processor,
    tokenizer::TokenizerFromTrain,
};

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
    let params = match get_call_message::<TokenizerFromTrain>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    if params.model.is_none() || params.trainer.is_none() {
        crate::set_empty_output!(out_ptr, out_len);
        return CallStatus::EmptyParams.into();
    }
    let model: tokenizers::models::ModelWrapper = match params.model.unwrap().try_into() {
        Ok(res) => res,
        Err(e) => {
            match e.1 {
                Some(e) => set_call_result(messages::Error { details: e }, out_ptr, out_len),
                None => crate::set_empty_output!(out_ptr, out_len),
            };
            return e.0.into();
        }
    };
    let mut trainer: tokenizers::models::TrainerWrapper = match params.trainer.unwrap().try_into() {
        Ok(res) => res,
        Err(e) => {
            match e.1 {
                Some(e) => set_call_result(messages::Error { details: e }, out_ptr, out_len),
                None => crate::set_empty_output!(out_ptr, out_len),
            };
            return e.0.into();
        }
    };
    let mut builder: TokenizerBuilder<
        ModelWrapper,
        NormalizerWrapper,
        PreTokenizerWrapper,
        PostProcessorWrapper,
        DecoderWrapper,
    > = TokenizerBuilder::new();
    builder = builder.with_model(model);
    if let Some(normalizer) = params.normalizer {
        builder = match unsafe { (normalizer as *const NormalizerWrapper).as_ref() } {
            Some(n) => builder.with_normalizer(Some(n.clone())),
            None => {
                set_call_result(
                    messages::Error {
                        details: "Invalid normalizer pointer".to_string(),
                    },
                    out_ptr,
                    out_len,
                );
                return CallStatus::InvalidPointerDetails.into();
            }
        }
    }
    if let Some(pre_tokenizer) = params.pre_tokenizer {
        builder = match unsafe { (pre_tokenizer as *const PreTokenizerWrapper).as_ref() } {
            Some(pt) => builder.with_pre_tokenizer(Some(pt.clone())),
            None => {
                set_call_result(
                    messages::Error {
                        details: "Invalid pre-tokenizer pointer".to_string(),
                    },
                    out_ptr,
                    out_len,
                );
                return CallStatus::InvalidPointerDetails.into();
            }
        }
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
    if let Err(e) = tk.train_from_files(&mut trainer, params.files) {
        set_call_result(
            messages::Error {
                details: e.to_string(),
            },
            out_ptr,
            out_len,
        );
        return CallStatus::TokenizerTrainingErrorDetails.into();
    }
    let pretty = params.pretty.unwrap_or(false);
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
    crate::set_empty_output!(out_ptr, out_len);
    CallStatus::Ok.into()
}
