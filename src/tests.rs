mod test {
    use crate::*;

    #[test]
    fn rot_word_test() {
        let word = [0x00, 0x01, 0x02, 0x03];
        let expected = [0x01, 0x02, 0x03, 0x00];
        assert_eq!(rot_word(word), expected);
    }

    #[test]
    fn sub_word_test() {
        let word = [0x01, 0xc2, 0x9e, 0x03];
        let expected = [0x7c, 0x25, 0x0b, 0x7b];
        assert_eq!(sub_word(word), expected);
    }
}
