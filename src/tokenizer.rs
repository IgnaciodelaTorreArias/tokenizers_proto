use tokenizers::tokenizer::{
    DecoderWrapper, EncodeInput, ModelWrapper, NormalizerWrapper, PostProcessorWrapper,
    PreTokenizerWrapper, Tokenizer, TokenizerImpl,
};

use crate::buffer_utils::{get_call_message, set_call_result};
use crate::messages::tokenizer::*;
use crate::messages::{self, CallStatus};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tokenizer_from_file(
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
    let params = match get_call_message::<TokenizerFromFile>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let tk = match Tokenizer::from_file(params.file) {
        Ok(res) => res.into_inner(),
        Err(e) => {
            set_call_result(
                messages::Error {
                    details: e.to_string(),
                },
                out_ptr,
                out_len,
            );
            return CallStatus::TokenizerLoadFileErrorDetails.into();
        }
    };
    let p = Box::new(tk);
    unsafe { *instance_ptr = Box::into_raw(p) }
    crate::set_empty_output!(out_ptr, out_len);
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn encode(
    instance_ptr: *const TokenizerImpl<
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
    let params = match get_call_message::<EncodeParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let tokenizer = match unsafe { instance_ptr.as_ref() } {
        Some(res) => res,
        None => {
            set_call_result(
                messages::Error {
                    details: "Invalid tokenizer pointer".to_string(),
                },
                out_ptr,
                out_len,
            );
            return CallStatus::InvalidPointerDetails.into();
        }
    };
    let include_type_ids = params.include_type_ids.unwrap_or(false);
    let include_tokens = params.include_tokens.unwrap_or(false);
    let include_words = params.include_words.unwrap_or(false);
    let include_offsets = params.include_offsets.unwrap_or(false);
    let include_special_tokens_mask = params.include_special_tokens_mask.unwrap_or(false);
    let include_attention_mask = params.include_attention_mask.unwrap_or(false);
    let include_overflowing = params.include_overflowing.unwrap_or(false);
    let input: EncodeInput = if let Some(input2) = params.input2 {
        (params.input, input2).into()
    } else {
        params.input.into()
    };
    let encoding = if include_offsets {
        tokenizer.encode(input, params.add_special_tokens)
    } else {
        tokenizer.encode_fast(input, params.add_special_tokens)
    };
    let mut original = match encoding {
        Ok(res) => res,
        Err(e) => {
            set_call_result(
                messages::Error {
                    details: e.to_string(),
                },
                out_ptr,
                out_len,
            );
            return CallStatus::TokenizerEncodingErrorDetails.into();
        }
    };
    let mut encode_result = vec![encoding_to_message(
        &original,
        include_type_ids,
        include_tokens,
        include_words,
        include_offsets,
        include_special_tokens_mask,
        include_attention_mask,
    )];
    if include_overflowing {
        encode_result.extend(original.take_overflowing().into_iter().map(|original| {
            encoding_to_message(
                &original,
                include_type_ids,
                include_tokens,
                include_words,
                include_offsets,
                include_special_tokens_mask,
                include_attention_mask,
            )
        }));
    };
    set_call_result(
        EncodeResult {
            encodings: encode_result,
        },
        out_ptr,
        out_len,
    );
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decode(
    instance_ptr: *const TokenizerImpl<
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
    let params = match get_call_message::<DecodeParams>(ptr, len) {
        Ok(res) => res,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let tokenizer = match unsafe { instance_ptr.as_ref() } {
        Some(res) => res,
        None => {
            set_call_result(
                messages::Error {
                    details: "Invalid tokenizer pointer".to_string(),
                },
                out_ptr,
                out_len,
            );
            return CallStatus::InvalidPointerDetails.into();
        }
    };
    let decode_result = match tokenizer.decode(&params.ids, params.skip_special_tokens) {
        Ok(res) => res,
        Err(e) => {
            set_call_result(
                messages::Error {
                    details: e.to_string(),
                },
                out_ptr,
                out_len,
            );
            return CallStatus::TokenizerDecodingErrorDetails.into();
        }
    };
    set_call_result(
        DecodeResult {
            decoded: decode_result,
        },
        out_ptr,
        out_len,
    );
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_tokenizer(
    ptr: *mut TokenizerImpl<
        ModelWrapper,
        NormalizerWrapper,
        PreTokenizerWrapper,
        PostProcessorWrapper,
        DecoderWrapper,
    >,
) {
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
