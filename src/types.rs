// Importando da biblioteca `serde` as funções de serialização e desserialização.
use serde::{Serialize, Deserialize};
// Carregando os macros de serialização, desserialização e debug.
#[derive(Serialize, Deserialize, Debug)]

/// Implementação da estrutura de entrada, esta Struct implementa métodos de serialização e deserialização da biblioteca serde
/// # Atributos
/// * `username` - uma string que representa o nome de usuário da conta a ser guardada
/// * `domain` - uma string que representa o domínio do site ao qual a conta pertence
/// * `password` - uma string que representa a senha da conta guardada
pub struct Entry {
    pub username: String,
    pub domain: String,
    pub password: String
}