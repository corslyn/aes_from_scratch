mod config;
mod tests;

fn main() {
    let state: [u8; 16] = "this is one text"
        .as_bytes()
        .try_into()
        .expect("state is not 16 bytes !");
    print_state(state);
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
    let mut modified = state.clone();
    for i in 0..16 {
        modified[i as usize] = config::SBOX[i as usize];
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
