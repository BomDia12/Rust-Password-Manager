/// Importando da biblioteca `serde` as funções de serialização e desserialização.
use serde::{Serialize, Deserialize};
/// Carregando os macros de serialização, desserialização e debug.
#[derive(Serialize, Deserialize, Debug)]

/// Implementação da estrutura de entrada, que terá:
/// # Atributos
/// * `username` - uma string que contém o nome de usuário da conta a ser guardada
/// * `domain` - domínio do site ao qual a conta pertence
/// * `password` - senha da conta guardada
pub struct Entry {
    pub username: String,
    pub domain: String,
    pub password: String
}