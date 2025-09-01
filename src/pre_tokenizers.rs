use tokenizers::{PreTokenizedString, PreTokenizer, PreTokenizerWrapper};

use crate::buffer_utils::{get_call_message, set_call_result};
use crate::messages::{self, CallStatus, pre_tokenizers::PreTokenizeParams};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pre_tokenize(
    instance_ptr: *mut PreTokenizerWrapper,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<PreTokenizeParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let mut pre_tokenized_string =
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
    let pre_tokenizer = match unsafe { instance_ptr.as_ref() } {
        Some(res) => res,
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
    };
    if let Err(e) = pre_tokenizer.pre_tokenize(&mut pre_tokenized_string) {
        set_call_result(
            messages::Error {
                details: e.to_string(),
            },
            out_ptr,
            out_len,
        );
        return CallStatus::PreTokenizationErrorDetails.into();
    };
    crate::set_empty_output!(out_ptr, out_len);
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
    let params = match get_call_message::<messages::pre_tokenizers::PreTokenizerWrapper>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let pre_tokenizer_wrapper: PreTokenizerWrapper = match params.try_into() {
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
    crate::set_empty_output!(out_ptr, out_len);
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_pre_tokenizer_wrapper(ptr: *mut PreTokenizerWrapper) {
    unsafe {
        drop(Box::from_raw(ptr));
    }
}
