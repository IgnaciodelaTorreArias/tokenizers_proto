use tokenizers::pre_tokenizer::PreTokenizedString;

use crate::buffer_utils::{get_call_message, set_call_result};
use crate::messages::pipeline_string::{
    PipelineStringParams, SplitParams, SplitResult
};
use crate::messages::{self, CallStatus, Offsets};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_splits(
    instance_ptr: *mut PreTokenizedString,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<SplitParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let instance = match unsafe { instance_ptr.as_mut() }{
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
    let offset_ref = match params.offset_referential().try_into() {
        Ok(res) => res,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::UnknownEnumValue.into();
        }
    };
    let offset_type = match params.offset_type().try_into() {
        Ok(res) => res,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::UnknownEnumValue.into();
        }
    };
    let (tokens, offsets) = instance
        .get_splits(offset_ref, offset_type)
        .into_iter()
        .map(|(str, (start, end), _)| (str.to_string(), Offsets{
            start: start as u64,
            end: end as u64,
        }))
        .unzip();
    let mut res = SplitResult {
        tokens: tokens,
        offsets: vec![],
    };
    if params.include_offsets {
        res.offsets = offsets;
    };
    set_call_result(res, out_ptr, out_len);
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_pipeline_string(
    instance_ptr: *mut *mut PreTokenizedString,
    ptr: *const u8,
    len: usize,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let params = match get_call_message::<PipelineStringParams>(ptr, len) {
        Ok(msg) => msg,
        Err(_) => {
            crate::set_empty_output!(out_ptr, out_len);
            return CallStatus::DecodeError.into();
        }
    };
    let pipeline_string: PreTokenizedString = params.into();
    let pipeline_string = Box::new(pipeline_string);
    unsafe {
        *instance_ptr = Box::into_raw(pipeline_string);
    }
    crate::set_empty_output!(out_ptr, out_len);
    CallStatus::Ok.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_pipeline_string(ptr: *mut PreTokenizedString){
    unsafe {
        drop(Box::from_raw(ptr));
    }
}