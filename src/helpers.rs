macro_rules! as_mut_ptr {
    ($($arg:tt)*) => {{
        Box::into_raw(Box::new($($arg)*))
    }}
}


// Convert C string `c_str` to a String. Return an empty string if
// `c_str` is NULL.
pub fn c_str_to_string(c_str: *const std::os::raw::c_char) -> String {
    if c_str.is_null() {
        String::new()
    } else {
        let bytes = unsafe { std::ffi::CStr::from_ptr(c_str as *const _).to_bytes() };

        String::from_utf8_lossy(bytes).to_string()
    }
}

pub fn string_to_c_str(s: &str) -> *const std::os::raw::c_char {
    s.to_string().as_ptr() as *const std::os::raw::c_char
}
