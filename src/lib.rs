use std::{os::raw::c_int, ptr};

mod percent_decode;
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

    // 预先搜索一边 % 如果没有%就不做url解码
    if let Ok(_) = rust_str.binary_search(&b'%') {
        let mut vec = Vec::with_capacity(length);
        // 所有权不在我们，切记，只能借用
        for element in &rust_str {
            vec.push(*element);
            // 几重编码就调用几次
            urldecode_and_push!(vec);
            urldecode_and_push!(vec);
        }
        // 如果被解码了，字符长度肯定不一致，所以强行覆盖
        if vec.len() != length {
            vec.push(0);
            unsafe {
                ptr::copy_nonoverlapping(vec.as_ptr(), c_str, length);
            }
        }
    }

    // inhibit compiler from automatically calling T’s destructor，所有权在c端，rust的rail释放会导致double free
    core::mem::forget(rust_str);
    return 0;
}

macro_rules! consume_token {
    ($characters:expr; $count:expr) => {{
        for _ in 0..$count {
            $characters.next();
        }
    }};
}

// 如果解码有问题等不做任何解码
macro_rules! rollback {
    ($vec:expr; $($chr:expr),*) => {
        $(
            $vec.push($chr);
        )*
    };
}

macro_rules! decode_token {
    ($iter:expr, $res:expr, $times:expr) => {
        // 消耗四个token，那就是unicode
        // 消耗两个token，那就是hex
        // 如果出了错，就不会消耗token
        let hex_digits: String = $iter.as_str().chars().take($times).collect();
        if let Ok(code_point) = u32::from_str_radix(&hex_digits, 16) {
            if let Some(unicode_char) = std::char::from_u32(code_point) {
                $res.push(unicode_char);
                // 只有unicode解码成功，才会消耗四个字符
                consume_token!($iter; $times);
                continue;
            }
        }
    };
}

pub fn unicode_decode(input: &str) -> Option<String> {
    let mut result = String::new(); // 用于存储解析后的字符串
    let mut characters = input.chars(); // 字符迭代器

    while let Some(c) = characters.next() {
        if c != '\\' {
            result.push(c);
        } else if let Some(escaped) = characters.next() {
            match escaped {
                'u' => {
                    decode_token!(characters, result, 4);
                    // 如果unicode解码失败，那么不做任何处理
                    rollback!(result; '\\', escaped);
                }
                'x' => {
                    decode_token!(characters, result, 2);
                    rollback!(result; '\\', escaped);
                }
                _ => {
                    rollback!(result; '\\', escaped);
                }
            }
        } else {
            return None; // 以'\'结尾的无效输入
        }
    }

    Some(result)
}
