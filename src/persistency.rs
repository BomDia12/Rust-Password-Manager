use crate::{types::Entry, encryption::{encrypt_data, decrypt_data}};
use std::fs;

pub fn save_data_to_disk(data: &Vec<Entry>) {
    let json = serde_json::to_string(&data).expect("Erro serializer");
    let encrypted_data = encrypt_data(json, "");
    fs::write("data", encrypted_data).expect("Erro escrevendo json");
}

pub fn read_data_from_disk(data: &mut Vec<Entry>) {
    let encrypted_data = match fs::read("data") {
        Ok(data) => data,
        Err(_) => return
    };
    let decrypted_data = decrypt_data(encrypted_data, "");
    let deserialized: Vec<Entry> = serde_json::from_str(&decrypted_data).expect("Erro deserialize");
    for i in deserialized {
        data.push(i)
    }
}