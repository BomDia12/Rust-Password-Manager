use sha2::{Sha256, Digest};
use aes::Aes256;
use block_modes::{BlockMode, Cfb};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;
use std::env;
pub fn encrypt_data(data: String, key) {
    let cipher = Aes256::new(&key);
    let plaintext = data.as_bytes();
    let plainTextSize = plaintext.len();
    let mut buffer = [0u8; 256];
    buffer[..plainTextSize].copy_from_slice(plaintext)
    let ciphertext = cipher.encrypt(&mut buffer, plainTextSize).unwrap();
    hex::encode(ciphertext)
}

pub fn decrypt_data(ciphertext, key) -> String {
    let cipher = Aes256::new(&key);
    let mut buf = ciphertext.to_vec();
    let decrypted = cipher.decrypt(&mut buf).unwrap();
    str::from_utf8(decrypted).unwrap()
}

pub fn generate_encryption_key<Sha256: Digest>(input_string: String) {
    let hash = Sha256::digest(input_string.as_bytes());
}   