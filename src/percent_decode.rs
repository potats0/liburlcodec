#[macro_export]
macro_rules! handle_hex {
    ($vec:expr) => {{
        if $vec.len() >= 3 {
            let second_hex = $vec.pop().unwrap();
            let first_hex = $vec.pop().unwrap();
            let h = match char::from(first_hex).to_digit(16) {
                Some(s) => s,
                None => {
                    $vec.push(first_hex);
                    $vec.push(second_hex);
                    continue;
                }
            };
            let l = match char::from(second_hex).to_digit(16) {
                Some(s) => s,
                None => {
                    $vec.push(first_hex);
                    $vec.push(second_hex);
                    continue;
                }
            };
            $vec.pop();
            $vec.push((h * 0x10 + l) as u8);
        }
    }}; // }
}

/// 将vector尾部的三个元素作为url编码的内容，解码并push回vector中
///
#[macro_export]
macro_rules! urldecode_and_push {
    ($vec:expr) => {{
        match $vec.get($vec.len().wrapping_sub(3)) {
            Some(s) => match *s {
                b'%' => {
                    handle_hex!($vec);
                }
                _ => continue,
            },
            None => continue,
        }
    }};
}
