use aes_from_scratch::*;
use rand::prelude::*;
fn main() {
    let key: [u8; 16] = hex::decode("aa000000000000000000000000000000")
        .unwrap()
        .try_into()
        .unwrap();

    println!("Original key: {:02x?}", key);

    let lambda_set = setup(key);
    if verify_set(&lambda_set) {
        println!("xor of all bytes = 0 : :)");
    } else {
        eprintln!("xor of all bytes != 0 :(");
    }
}

fn setup(key: [u8; 16]) -> Vec<[u8; 16]> {
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
        lambda_set.push(encrypt_with_rounds(plaintext, key, 3));
    }

    lambda_set
}

fn verify_set(lambda_set: &Vec<[u8; 16]>) -> bool {
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
