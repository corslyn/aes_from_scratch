use crate::config;

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

fn rcon(input: usize) -> [u8; 4] {
    [config::RCON[input], 0, 0, 0]
}

fn xor_words(a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

fn key_expansion(key: [u8; 16]) -> Vec<[u8; 16]> {
    let mut round_keys = Vec::with_capacity(11);

    let mut words: Vec<[u8; 4]> = Vec::with_capacity(44);

    for i in 0..4 {
        words.push([key[i * 4], key[i * 4 + 1], key[i * 4 + 2], key[i * 4 + 3]]);
    }

    for i in 4..44 {
        let mut temp = words[i - 1];

        if i % 4 == 0 {
            temp = sub_word(rot_word(temp));
            temp = xor_words(temp, rcon(i / 4));
        }

        words.push(xor_words(words[i - 4], temp));
    }

    for round in 0..11 {
        let mut round_key = [0u8; 16];
        for i in 0..4 {
            let word = words[round * 4 + i];
            round_key[i * 4..i * 4 + 4].copy_from_slice(&word);
        }
        round_keys.push(round_key);
    }

    round_keys
}

fn sub_bytes(state: [u8; 16]) -> [u8; 16] {
    let mut modified = [0u8; 16];
    for i in 0..16 {
        modified[i as usize] = config::SBOX[state[i] as usize];
    }
    modified
}

fn shift_rows(state: [u8; 16]) -> [u8; 16] {
    let mut modified = [0u8; 16];

    for row in 0..4 {
        for col in 0..4 {
            let new_col = (col + row) % 4;
            modified[row + 4 * col] = state[row + 4 * new_col];
        }
    }

    modified
}

fn mix_columns(state: [u8; 16]) -> [u8; 16] {
    let mut modified = [0u8; 16];
    for col in 0..4 {
        let a0 = state[col * 4];
        let a1 = state[col * 4 + 1];
        let a2 = state[col * 4 + 2];
        let a3 = state[col * 4 + 3];
        modified[col * 4] = config::MULT_2[a0 as usize] ^ config::MULT_3[a1 as usize] ^ a2 ^ a3;
        modified[col * 4 + 1] = a0 ^ config::MULT_2[a1 as usize] ^ config::MULT_3[a2 as usize] ^ a3;
        modified[col * 4 + 2] = a0 ^ a1 ^ config::MULT_2[a2 as usize] ^ config::MULT_3[a3 as usize];
        modified[col * 4 + 3] = config::MULT_3[a0 as usize] ^ a1 ^ a2 ^ config::MULT_2[a3 as usize];
    }
    modified
}

fn add_round_key(state: [u8; 16], round_key: [u8; 16]) -> [u8; 16] {
    std::array::from_fn(|i| state[i] ^ round_key[i])
}

pub fn encrypt(plaintext: [u8; 16], key: [u8; 16]) -> [u8; 16] {
    let keys = key_expansion(key);
    // pre-whitening
    let mut encrypted = add_round_key(plaintext, keys[0]);

    // round 1 to 10
    for round in 1..10 {
        encrypted = sub_bytes(encrypted);
        encrypted = shift_rows(encrypted);
        encrypted = mix_columns(encrypted);
        encrypted = add_round_key(encrypted, keys[round]);
    }

    // last round (no mixing)
    encrypted = sub_bytes(encrypted);
    encrypted = shift_rows(encrypted);
    encrypted = add_round_key(encrypted, keys[10]);

    encrypted
}

#[cfg(test)]
mod tests {
    use super::*;

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
    #[test]
    fn rcon_test() {
        let input = 67;
        let expected = [0x2f, 0, 0, 0];
        assert_eq!(rcon(input), expected);
    }

    #[test]
    fn key_expansion_test() {
        let key = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];

        let expected = [
            [
                0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
                0x4f, 0x3c,
            ],
            [
                0xa0, 0xfa, 0xfe, 0x17, 0x88, 0x54, 0x2c, 0xb1, 0x23, 0xa3, 0x39, 0x39, 0x2a, 0x6c,
                0x76, 0x05,
            ],
            [
                0xf2, 0xc2, 0x95, 0xf2, 0x7a, 0x96, 0xb9, 0x43, 0x59, 0x35, 0x80, 0x7a, 0x73, 0x59,
                0xf6, 0x7f,
            ],
            [
                0x3d, 0x80, 0x47, 0x7d, 0x47, 0x16, 0xfe, 0x3e, 0x1e, 0x23, 0x7e, 0x44, 0x6d, 0x7a,
                0x88, 0x3b,
            ],
            [
                0xef, 0x44, 0xa5, 0x41, 0xa8, 0x52, 0x5b, 0x7f, 0xb6, 0x71, 0x25, 0x3b, 0xdb, 0x0b,
                0xad, 0x00,
            ],
            [
                0xd4, 0xd1, 0xc6, 0xf8, 0x7c, 0x83, 0x9d, 0x87, 0xca, 0xf2, 0xb8, 0xbc, 0x11, 0xf9,
                0x15, 0xbc,
            ],
            [
                0x6d, 0x88, 0xa3, 0x7a, 0x11, 0x0b, 0x3e, 0xfd, 0xdb, 0xf9, 0x86, 0x41, 0xca, 0x00,
                0x93, 0xfd,
            ],
            [
                0x4e, 0x54, 0xf7, 0x0e, 0x5f, 0x5f, 0xc9, 0xf3, 0x84, 0xa6, 0x4f, 0xb2, 0x4e, 0xa6,
                0xdc, 0x4f,
            ],
            [
                0xea, 0xd2, 0x73, 0x21, 0xb5, 0x8d, 0xba, 0xd2, 0x31, 0x2b, 0xf5, 0x60, 0x7f, 0x8d,
                0x29, 0x2f,
            ],
            [
                0xac, 0x77, 0x66, 0xf3, 0x19, 0xfa, 0xdc, 0x21, 0x28, 0xd1, 0x29, 0x41, 0x57, 0x5c,
                0x00, 0x6e,
            ],
            [
                0xd0, 0x14, 0xf9, 0xa8, 0xc9, 0xee, 0x25, 0x89, 0xe1, 0x3f, 0x0c, 0xc8, 0xb6, 0x63,
                0x0c, 0xa6,
            ],
        ];

        assert_eq!(key_expansion(key), expected);
    }

    #[test]
    fn test_sub_bytes() {
        let state = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ];
        let expected = [
            0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7,
            0xab, 0x76,
        ];
        assert_eq!(sub_bytes(state), expected);
    }

    #[test]
    fn test_shift_rows() {
        let state = [
            0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7,
            0xab, 0x76,
        ];
        let expected = [
            0x63, 0x6b, 0x67, 0x76, 0xf2, 0x01, 0xab, 0x7b, 0x30, 0xd7, 0x77, 0xc5, 0xfe, 0x7c,
            0x6f, 0x2b,
        ];
        assert_eq!(shift_rows(state), expected);
    }

    #[test]
    fn test_mults() {
        assert_eq!(config::MULT_2[0x63], 0xc6);
        assert_eq!(config::MULT_3[0x63], 0xa5);
    }

    #[test]
    fn test_mix_columns() {
        let state = [
            0x63, 0x6b, 0x67, 0x76, 0xf2, 0x01, 0xab, 0x7b, 0x30, 0xd7, 0x77, 0xc5, 0xfe, 0x7c,
            0x6f, 0x2b,
        ];
        let expected = [
            0x6a, 0x6a, 0x5c, 0x45, 0x2c, 0x6d, 0x33, 0x51, 0xb0, 0xd9, 0x5d, 0x61, 0x27, 0x9c,
            0x21, 0x5c,
        ];
        assert_eq!(mix_columns(state), expected);
    }

    #[test]
    fn test_add_round_key() {
        let key = [
            0xd6, 0xaa, 0x74, 0xfd, 0xd2, 0xaf, 0x72, 0xfa, 0xda, 0xa6, 0x78, 0xf1, 0xd6, 0xab,
            0x76, 0xfe,
        ];
        let state = [
            0x6a, 0x6a, 0x5c, 0x45, 0x2c, 0x6d, 0x33, 0x51, 0xb0, 0xd9, 0x5d, 0x61, 0x27, 0x9c,
            0x21, 0x5c,
        ];
        let expected = [
            0xbc, 0xc0, 0x28, 0xb8, 0xfe, 0xc2, 0x41, 0xab, 0x6a, 0x7f, 0x25, 0x90, 0xf1, 0x37,
            0x57, 0xa2,
        ];
        assert_eq!(add_round_key(state, key), expected);
    }

    #[test]
    fn round_test() {
        let initial = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ];

        let expected = [
            0xbc, 0xc0, 0x28, 0xb8, 0xfe, 0xc2, 0x41, 0xab, 0x6a, 0x7f, 0x25, 0x90, 0xf1, 0x37,
            0x57, 0xa2,
        ];
        let key = [
            0xd6, 0xaa, 0x74, 0xfd, 0xd2, 0xaf, 0x72, 0xfa, 0xda, 0xa6, 0x78, 0xf1, 0xd6, 0xab,
            0x76, 0xfe,
        ];
        let mut calculated = sub_bytes(initial);
        calculated = shift_rows(calculated);
        calculated = mix_columns(calculated);
        calculated = add_round_key(calculated, key);
        assert_eq!(calculated, expected);
    }
}
