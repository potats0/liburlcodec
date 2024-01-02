use liburlcodec;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = liburlcodec::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_urlencoding() {
        liburlcodec::urldecode("aaaa".as_ptr(), 4);
        // assert_eq!(result, 4);
    }
}
