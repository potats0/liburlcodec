use std::{
    ffi::{c_char, CStr},
    os::raw::c_int,
};

type DebugFn = unsafe extern "C" fn(*const c_char, ...) -> c_int;

#[no_mangle]
pub extern "C" fn registerDebugFunc(debug_fn: DebugFn) -> c_int {
    unsafe {
        debug_fn(
            b"Hello, %s!\n\0".as_ptr() as *const c_char,
            b"World\0".as_ptr() as *const c_char,
        );
    }
    return 0;
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
