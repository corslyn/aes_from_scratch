mod config;
mod tests;

fn rot_word(word: [u8; 4]) -> [u8; 4] {
    [word[1], word[2], word[3], word[0]]
}

fn sub_word(word: [u8; 4]) -> [u8; 4] {
    [
        config::SBOX[word[0] as usize],
        config::SBOX[word[1] as usize],
        config::SBOX[word[2] as usize],
        config::SBOX[word[3] as usize],
    ]
}
