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

/** `urldecode` 将指定的c char做url多重解码
解码后的内容会覆盖原本的内存区域，如果解码不成功则不会覆盖。

```
    let mut str = String::from("%25%36%31%25%36%32%25%36%33%25%36%34%25%36%35%25%36%34");
    urlcodec::urldecode(str.as_mut_ptr(), str.len());
    assert_eq!(str , "abcded\0434\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
```
*/
#[no_mangle]
pub extern "C" fn urldecode(c_str: *mut u8, length: usize) -> c_int {
    let rust_str: Vec<u8> = unsafe { Vec::from_raw_parts(c_str, length, length) };

    let mut vec = Vec::with_capacity(length);
    // 所有权不在我们，切记，只能借用
    for element in &rust_str {
        vec.push(*element);
        // 几重编码就调用几次
        urldecode_and_push!(vec);
        urldecode_and_push!(vec);
        println!();
    }
    // 如果被解码了，字符长度肯定不一致，所以强行覆盖
    if vec.len() != length {
        vec.push(0);
        unsafe {
            ptr::copy_nonoverlapping(vec.as_ptr(), c_str, length);
        }
    }
    // inhibit compiler from automatically calling T’s destructor，所有权在c端，rust的rail释放会导致double free
    core::mem::forget(rust_str);
    return 0;
}
