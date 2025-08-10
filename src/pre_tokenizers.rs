use tokenizers::{
    OffsetReferential, OffsetType, PreTokenizedString, PreTokenizer, PreTokenizerWrapper,
};

use crate::buffer_utils::{get_call_message, set_call_result};
use crate::general_utils::get_sequence;
use crate::messages::pre_tokenizers::{
    self, OffsetReferential as OR, OffsetType as OT, PreTokenizeResult, PreTokenizerWrapperParams,
    pre_tokenizer_wrapper_params::Params,
};
use crate::messages::{self, CallStatus, ConversionError, Offsets};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pre_tokenize(
    instance_ptr: *mut PreTokenizerWrapper,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<pre_tokenizers::PreTokenizeParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let mut pretokenized = PreTokenizedString::from(params.normalized);
    let pre_tokenizer = unsafe { &*(instance_ptr as *mut dyn PreTokenizer) };
    if let Err(e) = pre_tokenizer.pre_tokenize(&mut pretokenized) {
        set_call_result(
            messages::Error {
                details: e.to_string(),
            },
            out_ptr,
            out_len,
        );
        return CallStatus::PreTokenizationErrorDetails.into();
    };
    let offset_ref = match OR::try_from(params.offset_referential) {
        Ok(res) => res,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::UnknownEnumValue.into();
        }
    };
    let offset_ref = match offset_ref {
        OR::UnknownReferential => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::UnknownEnumValue.into();
        }
        OR::Original => OffsetReferential::Original,
        OR::Normalized => OffsetReferential::Normalized,
    };
    let offset_type = match OT::try_from(params.offset_type) {
        Ok(res) => res,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::UnknownEnumValue.into();
        }
    };
    let offset_type = match offset_type {
        OT::UnknownType => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::UnknownEnumValue.into();
        }
        OT::Byte => OffsetType::Byte,
        OT::Char => OffsetType::Char,
        OT::None => OffsetType::None,
    };
    let mut res = PreTokenizeResult {
        tokens: pretokenized
            .get_splits(offset_ref, offset_type)
            .into_iter()
            .map(|(s, _, _)| s.to_string())
            .collect(),
        offsets: vec![],
    };
    if params.include_offsets {
        res.offsets = pretokenized
            .get_splits(offset_ref, offset_type)
            .into_iter()
            .map(|(_, (s, e), _)| Offsets {
                start: s as u64,
                end: e as u64,
            })
            .collect()
    }
    set_call_result(res, out_ptr, out_len);
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_pre_tokenizer_wrapper(
    instance_ptr: *mut *mut PreTokenizerWrapper,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<PreTokenizerWrapperParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    if params.params.is_none() {
        crate::set_empty_output!(out_ptr, out_len);
        return CallStatus::EmptyParams.into();
    }
    let pre_tokenizer_wrapper: PreTokenizerWrapper = match get_pre_tokenizer(params.params.unwrap())
    {
        Ok(res) => res,
        Err(e) => {
            match e.1 {
                Some(e) => set_call_result(messages::Error { details: e }, out_ptr, out_len),
                None => crate::set_empty_output!(out_ptr, out_len),
            };
            return e.0.into();
        }
    };
    let pre_tokenizer_wrapper = Box::new(pre_tokenizer_wrapper);
    unsafe {
        *instance_ptr = Box::into_raw(pre_tokenizer_wrapper);
    }
    CallStatus::Ok.into()
}

fn get_pre_tokenizer(pre_tokenizer: Params) -> Result<PreTokenizerWrapper, ConversionError> {
    Ok(match pre_tokenizer {
        Params::BertPreTokenizer(params) => params.into(),
        Params::ByteLevel(params) => params.into(),
        Params::Metaspace(params) => params.try_into()?,
        Params::Whitespace(params) => params.into(),
        Params::WhitespaceSplit(params) => params.into(),
        Params::Delimiter(params) => params.try_into()?,
        Params::Sequence(params) => {
            tokenizers::pre_tokenizers::sequence::Sequence::new(get_sequence(params)?).into()
        }
        Params::Split(params) => params.try_into()?,
        Params::Punctuation(params) => params.try_into()?,
        Params::Digits(params) => params.into(),
        Params::UnicodeScripts(params) => params.into(),
        Params::FixedLength(params) => params.into(),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_pre_tokenizer_wrapper(ptr: *mut PreTokenizerWrapper) {
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
