use urlcodec;

#[cfg(test)]
mod tests {

    use super::*;

    /*
    测试无url编码的情况
    */
    #[test]
    fn test_1() {
        let mut str = String::from("a");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str, "a");
    }

    /*
    测试url一次编码的解码情况
    */
    #[test]
    fn test_2() {
        let mut str = String::from("%41");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str, "A\01");
    }

    // 测试解码失败
    #[test]
    fn test_3() {
        let mut str = String::from("%R1");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str, "%R1");
    }

    // 测试解码失败
    #[test]
    fn test_4() {
        let mut str = String::from("%R");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str, "%R");
    }

    #[test]
    fn test_5() {
        let mut str = String::from("%%%");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str, "%%%");
    }

    // url 双重编码
    #[test]
    fn test_6() {
        let mut str = String::from("%25%34%31");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str, "A\0131\0\0\0\0");
    }

    // url 多重编码混合
    #[test]
    fn test_7() {
        let mut str = String::from("%%34%31");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str, "A\0131\0\0");
    }

    // url 多重编码混合
    #[test]
    fn test_8() {
        let mut str = String::from("%%31");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str, "%1\01");
    }
}
