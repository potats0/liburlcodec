use urlcodec;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        let a = "\\u0061";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("a".to_string()));
    }

    #[test]
    fn t2() {
        let a = "\\x61";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("a".to_string()));
    }

    // 测试解码失败
    #[test]
    fn t3() {
        let a = "abcdefg";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("abcdefg".to_string()));
    }

    #[test]
    fn t4() {
        let a = "abc\\x61defg";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("abcadefg".to_string()));
    }

    // 测试hex解码中乱入其他东西
    #[test]
    fn t5() {
        let a = "abc\\xR1defg";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("abc\\xR1defg".to_string()));
    }

    // 测试\字符后面非预期情况
    #[test]
    fn t6() {
        let a = "abc\\aR1defg";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("abc\\aR1defg".to_string()));
    }

    //
    #[test]
    fn t7() {
        let a = "abc\\u0061defg";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("abcadefg".to_string()));
    }

    #[test]
    fn t8() {
        let a = "abc\\u00p1defg";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("abc\\u00p1defg".to_string()));
    }

    #[test]
    fn t9() {
        let a = "abc\\uabrgdefg";
        let b = urlcodec::unicode_decode(a);
        assert_eq!(b, Some("abc\\uabrgdefg".to_string()));
    }
}
