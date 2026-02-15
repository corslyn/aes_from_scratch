mod config;
mod tests;

use hex;

fn main() {
    let plaintext: [u8; 16] = "theblockbreakers"
        .as_bytes()
        .try_into()
        .expect("state is not 16 bytes !");
    let key: [u8; 16] = hex::decode("2b7e151628aed2a6abf7158809cf4f3c")
        .unwrap()
        .try_into()
        .unwrap();
    print_state(encrypt(plaintext, key));
}

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

fn print_state(state: [u8; 16]) {
    for row in 0..4 {
        for col in 0..4 {
            print!("{:02x} ", state[(row + 4 * col) as usize]);
        }
        println!();
    }
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

fn encrypt(plaintext: [u8; 16], key: [u8; 16]) -> [u8; 16] {
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
