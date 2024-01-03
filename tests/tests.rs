use urlcodec;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_urlencoding() {
        let mut str = String::from("%25%36%31%25%36%32%25%36%33%25%36%34%25%36%35%25%36%34");
        urlcodec::urldecode(str.as_mut_ptr(), str.len());
        assert_eq!(str , "abcded\0434\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        println!("in test_urlencoding {:?}", str);
    }
}
