use crate::messages::{CallStatus, ConversionError, Sequence};

#[macro_export]
macro_rules! set_empty_output {
    ($out_ptr:expr, $out_len:expr) => {
        unsafe {
            *$out_ptr = std::ptr::null_mut();
            *$out_len = 0;
        }
    };
}

pub(crate) fn get_sequence<T>(params: Sequence) -> Result<Vec<T>, ConversionError>
where
    T: Clone,
{
    if params.addresses.len() == 0 {
        return Err((
            CallStatus::InvalidArgumentsDetails,
            Some("Sequence cannot be empty".to_string()),
        ));
    }
    if params.addresses.iter().any(|&p| (p as *const T).is_null()) {
        return Err((
            CallStatus::InvalidPointerDetails,
            Some("Null pointer found in sequence".to_string()),
        ));
    }
    let v: Vec<T> = params
        .addresses
        .into_iter()
        .map(|p | unsafe { &*(p as *const T) } )
        .cloned()
        .collect();
    Ok(v)
}
