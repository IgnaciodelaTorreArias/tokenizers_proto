use tokenizers::{Normalizer, NormalizerWrapper, PreTokenizedString};

use crate::buffer_utils::{get_call_message, set_call_result};
use crate::messages::{self, CallStatus, normalizers::NormalizeParams};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn normalize(
    instance_ptr: *mut NormalizerWrapper,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<NormalizeParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let pre_tokenized_string =
        match unsafe { (params.pipeline_string as *mut PreTokenizedString).as_mut() } {
            Some(res) => res,
            None => {
                set_call_result(
                    messages::Error {
                        details: "Invalid PreTokenizedString pointer".to_string(),
                    },
                    out_ptr,
                    out_len,
                );
                return CallStatus::InvalidPointerDetails.into();
            }
        };
    let normalizer = match unsafe { instance_ptr.as_ref() } {
        Some(res) => res,
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
    };
    if let Err(e) = pre_tokenized_string.normalize(|s| normalizer.normalize(s)) {
        set_call_result(
            messages::Error {
                details: e.to_string(),
            },
            out_ptr,
            out_len,
        );
        return CallStatus::NormalizationErrorDetails.into();
    };
    crate::set_empty_output!(out_ptr, out_len);
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
    let params = match get_call_message::<messages::normalizers::NormalizerWrapper>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let normalizer_wrapper: NormalizerWrapper = match params.try_into() {
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
    crate::set_empty_output!(out_ptr, out_len);
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_normalizer_wrapper(ptr: *mut NormalizerWrapper) {
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
