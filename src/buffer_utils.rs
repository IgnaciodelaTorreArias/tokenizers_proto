use std::mem::ManuallyDrop;

use prost::DecodeError;

pub(crate) fn get_call_message<T>(ptr: *const u8, len: usize) -> Result<T, DecodeError>
where
    T: Default + prost::Message,
{
    if ptr.is_null() || len == 0 {
        return Ok(T::default());
    }
    let input = unsafe { std::slice::from_raw_parts(ptr, len) };
    T::decode(input)
}

pub(crate) fn set_call_result<T: prost::Message>(
    res: T,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) {
    let l = res.encoded_len();
    if l == 0 {
        crate::set_empty_output!(out_ptr, out_len);
        return;
    }
    let buf = res.encode_to_vec();
    let mut buf = ManuallyDrop::new(buf);
    unsafe {
        *out_ptr = buf.as_mut_ptr();
        *out_len = buf.capacity();
    };
}

/// # Safety
/// Function must be called after a function that has an output.
/// With the same address and len the output was pointed to.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_buffer(ptr: *mut u8, len: usize) {
    unsafe { Vec::from_raw_parts(ptr, len, len) };
}
