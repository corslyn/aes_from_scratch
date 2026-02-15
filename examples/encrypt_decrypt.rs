use aes_from_scratch::decrypt;
use aes_from_scratch::encrypt;

fn main() {
    let plaintext: [u8; 16] = "Choucroute !!!!!"
        .as_bytes()
        .try_into()
        .expect("plaintext is not 16 bytes !");
    let key: [u8; 16] = hex::decode("44666c2dab8af940ea00076487d462a6")
        .unwrap()
        .try_into()
        .unwrap();
    println!("Plaintext : {}", std::str::from_utf8(&plaintext).unwrap());
    let encrypted = encrypt(plaintext, key);
    print!("Using key : ");
    for byte in key {
        print!("{:02x}", byte);
    }
    println!();
    print!("Encrypted : ");
    for byte in encrypted {
        print!("{:02x}", byte);
    }
    println!();

    println!(
        "Decrypted : {}",
        std::str::from_utf8(&decrypt(encrypted, key)).expect("not a valid utf8")
    );
}
