pub use aes_from_scratch::{config::SBOXI, *};
use rand::prelude::*;

pub fn setup(key: [u8; 16]) -> Vec<[u8; 16]> {
    let mut lambda_set = Vec::new();
    let mut rng = rand::rng();

    // random base
    let mut base_plaintext = [0u8; 16];
    for i in 1..16 {
        base_plaintext[i] = rng.random();
    }

    for i in 0..=255 {
        let mut plaintext = base_plaintext;
        // active byte is first byte (i = 0)
        plaintext[0] = i;
        lambda_set.push(encrypt_with_rounds(plaintext, key, 4)); // 4 rounds !!!
    }

    lambda_set
}

pub fn verify_set(lambda_set: &Vec<[u8; 16]>) -> bool {
    for byte_pos in 0..16 {
        let mut xor_sum = 0u8;
        for block in lambda_set {
            xor_sum ^= block[byte_pos];
        }
        if xor_sum != 0 {
            return false;
        }
    }
    true
}

pub fn reverse_state(lambda_set: &Vec<[u8; 16]>, guess: u8, guess_pos: usize) -> Vec<[u8; 16]> {
    let mut reversed = Vec::with_capacity(lambda_set.len());

    for element in lambda_set {
        let mut state = *element;
        state[guess_pos] ^= guess; // inverse add round key on byte

        // inverse shift row on byte
        let row = guess_pos % 4;
        let col = guess_pos / 4;

        let original_col = (col + row) % 4;
        let original_pos = original_col * 4 + row;

        let shifted_byte = state[guess_pos];
        state[guess_pos] = 0;
        state[original_pos] = shifted_byte;

        // inverse subbytes
        state[original_pos] = SBOXI[state[original_pos] as usize];

        reversed.push(state);
    }

    reversed
}

pub fn check_key_guess(guess: u8, reversed_set: &Vec<[u8; 16]>, byte_pos: usize) -> Option<u8> {
    let row = byte_pos % 4;
    let col = byte_pos / 4;

    let original_col = (col + row) % 4;
    let original_pos = original_col * 4 + row;

    let mut xor_sum = 0u8;

    for state in reversed_set {
        xor_sum ^= state[original_pos];
    }

    if xor_sum == 0 {
        println!("possible at {}: {:02x}", byte_pos, guess);
        Some(guess)
    } else {
        None
    }
}

pub fn recover_master_key(last_round_key: [u8; 16]) -> [u8; 16] {
    let mut words = vec![[0u8; 4]; 20]; // 4 rounds * 4 bytes + round 0 = 4*4 + 4 = 20
    for i in 0..4 {
        words[16 + i] = [
            last_round_key[i * 4],
            last_round_key[i * 4 + 1],
            last_round_key[i * 4 + 2],
            last_round_key[i * 4 + 3],
        ];
    }

    for i in (4..20).rev() {
        let temp = words[i];

        if i % 4 == 0 {
            let mut g = rot_word(words[i - 1]);
            g = sub_word(g);
            let r = rcon(i / 4);
            words[i - 4] = xor_words(temp, xor_words(g, r));
        } else {
            words[i - 4] = xor_words(temp, words[i - 1]);
        }
    }

    let mut master_key = [0u8; 16];
    for i in 0..4 {
        master_key[i * 4..i * 4 + 4].copy_from_slice(&words[i]);
    }

    master_key // MASTER ! MASTER !
}
