mod square;
use crate::square::*;

fn main() {
    let key: [u8; 16] = hex::decode("fbdb67135bfe6322f5361e5ac2671623") // round 4 key at byte 0 should be 44
        .unwrap()
        .try_into()
        .unwrap();
    // last round key = 4483ed3987ef15c3751b75b27e14ee2b
    println!("Original key: {:02x?}", key);

    let mut last_round_key = [0u8; 16];

    let lambda_set = setup(key);

    for byte_pos in 0..16 {
        let mut guesses = vec![];
        for guess in 0..=255 {
            let reversed = reverse_state(&lambda_set, guess, byte_pos);
            if let Some(guess) = check_key_guess(guess, &reversed, byte_pos) {
                guesses.push(guess);
            }
        }

        while guesses.len() > 1 {
            let lambda_set = setup(key); // new random set

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

    assert_eq!(master, key);
    println!("well guys we did it");
}
