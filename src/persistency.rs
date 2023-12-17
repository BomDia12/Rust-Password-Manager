/// Importando o tipo `Entry` para entrada de dados e as funções de codificação e decodificação de criptografia.
use crate::{types::Entry, encryption::{encrypt_data, decrypt_data}};

/// Importando métodos para sistema de arquivos (leitura e escrita).
use std::fs;

/// Função que irá salvar nossos dados de domínio, usuário e senha codificados em um arquivo na memória.
/// Primeiro, serializamos os dados para um json no formato de String.
/// Depois, transformamos essa string para bytes.
/// Então, utilizamos a função [encrypt_data] para codificar os dados.
/// E, por último, escrevemos esses dados codificados em um arquivo.
/// 
/// # Parâmetros
/// * `data` - Uma variável para a referência de um vetor de dados do tipo [Entry] que conterá os dados a serem codificados e guardados na memória.
/// * `key` - Uma variável que representa a chave de encriptação, em formato de array de bytes.
pub fn save_data_to_disk(data: &Vec<Entry>, key: &[u8]) {
    let json = serde_json::to_string(&data).expect("Erro serializer");
    let json = json.as_bytes();
    let encrypted_data = encrypt_data(json, key).unwrap();
    fs::write("data", encrypted_data).expect("Erro escrevendo json");
}

/// Função para leitura de arquivo codificado para decodificação e armazenamento em variável.
/// Primeiro, lemos os dados do arquivo. Caso o arquivo não exista, retornamos.
/// Então, utilizamos a função [decrypt_data] para decodificar os dados.
/// Após isso, desserializamos os dados para voltar a ser um vetor de dados do tipo [Entry].
/// E, por último, armazenamos esses dados na variável `data`.
/// 
/// # Parâmetros
/// * `key` - Chave para desencriptação dos dados guardados em arquivo.
pub fn read_data_from_disk(key: &[u8]) -> Result<Vec<Entry>, ()> {
    let encrypted_data = match fs::read("data") {
        Ok(data) => data,
        Err(_) => return Err(())
    };
    let decrypted_data = decrypt_data(&encrypted_data, key);
    let decrypted_data = match decrypted_data {
        Ok(a) => a,
        Err(_) => return Err(())
    };
    let deserialized: Vec<Entry> = match serde_json::from_slice(&decrypted_data) {
        Ok(data) => data,
        Err(_) => return Err(())
    };
    let mut data = Vec::new();
    for i in deserialized {
        data.push(i);
    }
    Ok(data)
}