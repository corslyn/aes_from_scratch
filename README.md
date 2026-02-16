# Homemade AES

[![CI](https://github.com/corslyn/aes_from_scratch/actions/workflows/rust.yml/badge.svg)](https://github.com/corslyn/aes_from_scratch/actions/workflows/rust.yml)
![Made With](https://img.shields.io/badge/made%20with-Rust-orange?logo=rust)
![Security](https://img.shields.io/badge/security-none-red)
![Crates.io](https://img.shields.io/badge/published-no-lightgrey)
![Maintenance](https://img.shields.io/badge/maintained-yes-brightgreen)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](../../pulls)
[![Stars](https://img.shields.io/github/stars/corslyn/aes_from_scratch?style=social)](https://github.com/corslyn/aes_from_scratch)

Some homemade, ~freshly grown~ AES ! (do not use)

Now with 4 rounds attack !

## Build
```
git clone https://github.com/corslyn/aes_from_scratch.git
cd aes_from_scratch
cargo build
```

Run tests:
```
cargo test
```
## Examples

Encrypts a plaintext using a key, and decrypts it afterwards

```
cargo run --example encrypt_decrypt
```

Executes the square attack on 4 rounds AES

```
cargo run --example square_attack
```
### Usage as a crate

For the square attack :
```rust
use aes_from_scratch::attacks::square::*;
use rand::prelude::*;

fn main() {

    let mut last_round_key = [0u8; 16];

    let lambda_set = setup();

    for byte_pos in 0..16 {
        let mut guesses = vec![];
        for guess in 0..=255 {
            let reversed = reverse_state(&lambda_set, guess, byte_pos);
            if let Some(guess) = check_key_guess(guess, &reversed, byte_pos) {
                guesses.push(guess);
            }
        }

        while guesses.len() > 1 {
            let lambda_set = setup(&mut stream); // new random set

            guesses.retain(|&guess| {
                let reversed = reverse_state(&lambda_set, guess, byte_pos);
                check_key_guess(guess, &reversed, byte_pos).is_some()
            });
        }
        last_round_key[byte_pos] = guesses[0];
        println!("last-round key byte {} = {:02x}", byte_pos, guesses[0]);
    }
    println!("Recovered last-round key: {:02x?}", last_round_key);
    let master = recover_master_key(last_round_key);
    println!("MASTER KEY: {:02x?}", master);
}

fn setup() -> Vec<[u8; 16]> {
    let mut lambda_set = Vec::new();
    let mut rng = rand::rng();

    let mut base_plaintext = [0u8; 16];
    for i in 1..16 {
        base_plaintext[i] = rng.random();
    }

    for i in 0..=255 {
        let mut plaintext = base_plaintext;
        plaintext[0] = i;
        lambda_set.push(encrypt_with_unknown_key(plaintext));
    }

    lambda_set
}

fn encrypt_with_unknown_key(plaintext: [u8;16]) -> [u8;16] {
    unimplemented!("Write the function that encrypts your plaintext !");
}
```

## Credits

https://web.archive.org/web/20230428141954/https://www.davidwong.fr/blockbreakers/aes.html
