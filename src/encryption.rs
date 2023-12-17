//! # Visão Geral - AES
//! Advanced Encryption Standard (aes) também conhecida como *cifra de Rijndael*, é uma cifra de blocos
//! de tamanho fixo simétrica, ou seja, divide a mensagem (texto que deseja-se cifrar) em blocos de N bits
//! no qual realiza o processo de cifração/decifração, no qual ambos possuem mesma chave. N pode possuir os valores de 128, 194 ou 256 bits.
//! Nas funções abaixo, a versão implementada da cifra de Rijndael foi a de 256 bits.
//! Uma de suas vantagens diante aos algortimos de cifração que o sucedeu é o tamanho N bits que ataques de força bruta sejam inviáveis para
//! a quebra de sigílo.
//! 
//! ## Blocos e rodadas
//! A mensagem é dividida em blocos de N bits e cada um desses blocos é aplicado o algoritmo de cifração e decifração da cifra de Rinjdael.
//! 
//! Tanto para cifrar tanto para decifrar, o aes executa o mesmo processo dividido em rodadas, diferem-se uma da outra apenas em relação a ordem dos processos executados.
//! O número de rodadas a ser aplicado varia conforme o número de bits escolhido:
//! - 10 para 128 bits
//! - 12 para 194 bits
//! - 14 para 256 bits
//! 
//! Em cada uma delas, 4 métodos diferentes são aplicados sob o bloco a fim de produzir "maior aleatoriedade" no resultado final. O bloco possui um "estado" que é alterado a cada passo
//! do algoritmo e é interpretado como uma matriz, no qual cada item da matriz refere-se a um byte.
//! 
//! ### Métodos
//! #### AddRoundKey
//! Cada byte do estado é combinado com um byte da *chave de rodada* utilizando da operação XOR bit a bit.
//! 
//! #### SubBytes
//! Substituição não linear dos bytes utilizando-se de uma tabela de pesquisa, por padrão a **S-Box**
//! 
//! #### ShiftRows
//! Opera nas linhas do estado, fazendo permutação dos bytes de uma mesma linha
//! 
//! #### MixColumns
//! Fornece difusão na cifra, no qual cada coluna da matriz é transformada utilizando-se de uma matriz fixa. As entradas
//! são tratadas como polinômios sobre GF (2⁸).
//! 
//! ## Cifração e decifração por rodada
//! O processo de cifração ocorre na seguinte ordem:
//! - SubBytes
//! - ShiftRows
//! - MixColumns
//! - AddRoundKey
//! Com exceção do último round que não faz a operação de mix columns
//! 
//! O processo de decifração ocorre na seguinte ordem:
//! - Inverse ShiftRows
//! - Inverse SubBytes
//! - AddRoundKey
//! - Inverse MixColumns
//! Com exceção do último round que não faz a operação de inverse mix columns


/// Importação das bibliotecas utilizadas para as funções de gerar hash de 256 bits e implementação cifração e decifração aes
use openssl::error::ErrorStack;
use openssl::sha::Sha256;
use openssl::symm::{encrypt, Cipher, decrypt};


/// # Função de encriptar 
/// ## Parâmetros
/// * `data`: toda a mensagem que deseja-se cifrar, já no formato de bytes
/// * `key`: chave de 256 bits que será usada 
/// ## Implementação
/// Chama o construtor cipher da biblioteca openssl para termos uma instância da
/// cifra de Rinjdael de 256 bits
/// A variável *iv* é o vetor de inicialização aleatório de 256 bits que faz xor bit a bit no processo inicial de cifração
/// a fim de aumentar a aleatoriedade da cifração.
/// Por fim chamamos a função encrypt da biblioteca openssl que faz todo o processo descrito anteriormente.
/// ## Retorno
/// Retornamos o texto cifrado utilizando a cifra de blocos aes 256 bits.
pub fn encrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_256_cbc();
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let ciphertext = encrypt(
        cipher,
        key,
        Some(iv),
        data);
    ciphertext
}

/// # Função de decifrar 
/// ## Parâmetros
/// * `data`: todo o ciphertext que deseja-se decifrar, já no formato de bytes
/// * `key`: chave de 256 bits, deve ser a mesma para decifrar a mensagem.
/// ## Implementação
/// Chama o construtor cipher da biblioteca openssl para termos uma instância da
/// cifra de Rinjdael de 256 bits
/// A variável *iv* é o vetor de inicialização aleatório de 256 bits utilizado no processo de cifração para aumentar a aleatoriedade,
/// aqui será usado para fazermos o processo inverso.
/// Por fim chamamos a função decrypt da biblioteca openssl que faz todo o processo de decifração descrito anteriormente.
/// ## Retorno
/// Retornamos a mensagem original que havia sido cifrada com aquela chave utilizando-se da cifra de blocos aes 256 bits.
pub fn decrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_256_cbc();
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
    let message = decrypt(
        cipher,
        key,
        Some(iv),
        data);
    message 
}

/// # Função para gerar hash de 256 bits 
/// ## Parâmetros
/// * `input_password`: string sob a qual será aplicada a função de hash
/// ## Implementação
/// Chama o construtor Sha256 da biblioteca openssl para termos uma instância da
/// classe Sha256;
/// Chama a função update do hash que receberá input_password no formato de bytes
/// Por fim chamamos a função finish da biblioteca openssl para indicar que o "hasheamento" está completo
/// Retornamos um vetor de inteiros de tamanho 32 que posteriormente será usado como a chave tanto de cifração tanto decifração
/// da cifra de Rinjdael
pub fn generate_key(input_password: &str) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(input_password.as_bytes());
    let res = hash.finish();
    res
}
