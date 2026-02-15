# Homemade AES

[![CI](https://github.com/corslyn/aes_from_scratch/actions/workflows/rust.yml/badge.svg)](https://github.com/corslyn/aes_from_scratch/actions/workflows/rust.yml)
![Made With](https://img.shields.io/badge/made%20with-Rust-orange?logo=rust)
![Security](https://img.shields.io/badge/security-none-red)
![Crates.io](https://img.shields.io/badge/published-no-lightgrey)
![Maintenance](https://img.shields.io/badge/maintained-yes-brightgreen)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](../../pulls)
[![Stars](https://img.shields.io/github/stars/corslyn/aes_from_scratch?style=social)](https://github.com/corslyn/aes_from_scratch)

Some homemade, ~freshly grown~ AES ! (do not use)

Implementing this for square attack implementation...

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