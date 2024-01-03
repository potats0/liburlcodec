use std::{os::raw::c_int, ptr};

mod percent_decode;

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
pub extern "C" fn urldecode(c_str: *mut u8, length: usize) -> c_int {
    let rust_str: Vec<u8> = unsafe { Vec::from_raw_parts(c_str, length, length) };

    let mut vec = Vec::with_capacity(length);
    // 所有权不在我们，切记，只能借用
    for element in &rust_str {
        vec.push(*element);
        match vec.get(vec.len().wrapping_sub(3)) {
            Some(s) => match *s {
                b'%' => {
                    handle_hex!(vec);
                    // 如果想解决多重url编码，那么就多调用几次
                    handle_hex!(vec);
                }
                _ => continue,
            },
            None => continue,
        }
    }
    // 如果被解码了，字符长度肯定不一致，所以强行覆盖
    if vec.len() != length {
        vec.push(0);
        unsafe {
            ptr::copy_nonoverlapping(vec.as_ptr(), c_str, length);
        }
    }
    // inhibit compiler from automatically calling T’s destructor
    core::mem::forget(rust_str);
    return 0;
}
