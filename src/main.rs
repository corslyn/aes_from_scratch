mod config;
mod aes;

use aes::encrypt;

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

fn print_state(state: [u8; 16]) {
    for row in 0..4 {
        for col in 0..4 {
            print!("{:02x} ", state[(row + 4 * col) as usize]);
        }
        println!();
    }
}


