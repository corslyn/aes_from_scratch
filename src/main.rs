fn rot_word(word: [u8; 4]) -> [u8; 4] {
    [word[1], word[2], word[3], word[0]]
}

mod test {
    use super::*;

    #[test]
    fn rot_word_test() {
        let word = [0x00, 0x01, 0x02, 0x03];
        let expected = [0x01, 0x02, 0x03, 0x00];
        assert_eq!(rot_word(word), expected);
    }
}
