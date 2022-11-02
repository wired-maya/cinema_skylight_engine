use std::{os::raw::c_void, slice, ffi::CString};

pub extern "system" fn debug_message_callback(
    source: u32,
    type_: u32,
    _id: u32,
    severity: u32,
    length: i32,
    message: *const i8,
    _user_param: *mut c_void
) {
    let type_str = match type_ {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED_BEHAVIOR",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED_BEHAVIOR",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_MARKER => "MARKER",
        gl::DEBUG_TYPE_PUSH_GROUP => "PUSH_GROUP",
        gl::DEBUG_TYPE_POP_GROUP => "POP_GROUP",
        gl::DEBUG_TYPE_OTHER => "OTHER",
        _ => "OTHER"
    };
    let source_str = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW_SYSTEM",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER_COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD_PARTY",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_OTHER => "OTHER",
        _ => "OTHER"
    };
    let severity_str = match severity {
        gl::DEBUG_SEVERITY_HIGH => "HIGH_SEVERITY",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM_SEVERITY",
        gl::DEBUG_SEVERITY_LOW => "LOW_SEVERITY",
        gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
        _ => "UNKNOWN_SEVERITY"
    };
    let message_cstr = unsafe {
        let buffer = slice::from_raw_parts(message as *const u8, length as usize);
        CString::from_vec_unchecked(buffer.to_vec())
    };

    println!("{}::{}::{}::{}", type_str, source_str, severity_str, message_cstr.to_str().unwrap());
}