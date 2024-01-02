use std::{ffi::CStr, os::raw::c_int};

#[repr(C)]
#[derive(Debug)]
pub enum CodecType {
    URL,
    QUERYSTRING,
}

#[no_mangle]
pub extern "C" fn get_enum() -> CodecType {
    CodecType::URL
}

#[no_mangle]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[no_mangle]
pub extern "C" fn urldecode(c_str: *const u8, length: usize) -> c_int {
    // 转换 C 字符串为 Rust 字符串
    let rust_str = unsafe {
        let c_str_slice = std::slice::from_raw_parts(c_str as *const u8, length);
        let c_str = CStr::from_bytes_with_nul_unchecked(c_str_slice);
        match c_str.to_str() {
            Ok(str_ref) => str_ref,
            Err(_) => {
                return -1;
            }
        }
    };
    println!("{}", rust_str);
    return 0;
}
