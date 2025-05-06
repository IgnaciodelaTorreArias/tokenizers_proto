use tokenizers::{NormalizedString, Normalizer, NormalizerWrapper};

use crate::buffer_utils::{get_call_message, set_call_result};
use crate::general_utils::get_sequence;
use crate::messages::normalizers::{
    self, NormalizerWrapperParams, normalizer_wrapper_params::Params,
};
use crate::messages::{self, CallStatus, ConversionError};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn normalize(
    instance_ptr: *mut NormalizerWrapper,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<normalizers::NormalizeParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let mut normalized = NormalizedString::from(params.original);
    let normalizer = unsafe { &*(instance_ptr as *mut dyn Normalizer) };
    if let Err(e) = normalizer.normalize(&mut normalized) {
        set_call_result(
            messages::Error {
                details: e.to_string(),
            },
            out_ptr,
            out_len,
        );
        return CallStatus::NormalizationErrorDetails.into();
    };
    set_call_result(
        normalizers::NormalizeResult {
            normalized: normalized.get().to_string(),
        },
        out_ptr,
        out_len,
    );
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_normalizer_wrapper(
    instance_ptr: *mut *mut NormalizerWrapper,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<NormalizerWrapperParams>(ptr, len) {
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
    let normalizer_wrapper: NormalizerWrapper = match get_normalizer(params.params.unwrap()) {
        Ok(res) => res,
        Err(e) => {
            match e.1 {
                Some(e) => set_call_result(messages::Error { details: e }, out_ptr, out_len),
                None => crate::set_empty_output!(out_ptr, out_len),
            };
            return e.0.into();
        }
    };
    let normalizer_wrapper = Box::new(normalizer_wrapper);
    unsafe {
        *instance_ptr = Box::into_raw(normalizer_wrapper);
    }
    CallStatus::Ok.into()
}

fn get_normalizer(normalizer: Params) -> Result<NormalizerWrapper, ConversionError> {
    Ok(match normalizer {
        Params::BertNormalizer(params) => params.into(),
        Params::Nfd(params) => params.into(),
        Params::Nfkd(params) => params.into(),
        Params::Nfc(params) => params.into(),
        Params::Nfkc(params) => params.into(),
        Params::Nmt(params) => params.into(),
        Params::StripNormalizer(params) => params.into(),
        Params::StripAccents(params) => params.into(),
        Params::Sequence(params) => {
            tokenizers::normalizers::Sequence::new(get_sequence(params)?).into()
        }
        Params::Lowercase(params) => params.into(),
        Params::Prepend(params) => params.into(),
        Params::Replace(params) => params.try_into()?,
        Params::Precompiled(params) => params.try_into()?,
        Params::ByteLevel(params) => params.into(),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_normalizer_wrapper(ptr: *mut NormalizerWrapper) {
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
