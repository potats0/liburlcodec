mod percent_decode {
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
        }};
    }
}
