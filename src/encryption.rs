use openssl::sha::Sha256;
use openssl::symm::{encrypt, Cipher, decrypt};

pub fn encrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, _> {
    let cipher = Cipher::aes_256_cbc();
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let ciphertext = encrypt(
        cipher,
        key,
        Some(iv),
        data);
    ciphertext
}

pub fn decrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, _> {
    let cipher = Cipher::aes_256_cbc();
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let message = decrypt(
        cipher,
        key,
        Some(iv),
        data);
    message 
}

pub fn generate_key(input_password: &str) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(input_password.as_bytes());
    let res = hash.finish();
    res
}