pub fn encrypt_data(data: String) -> String {
    data
}

pub fn decrypt_data(data: Vec<u8>) -> String {
    String::from_utf8(data).expect("não converteu")
}

pub fn generate_encryption_key() {}