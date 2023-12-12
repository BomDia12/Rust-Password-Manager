/// Importação do `replace` para garantia de todos os tipos de caracteres na senha sugerida.
use std::mem::replace;

/// Importação das funções de número aleatório para a geração de senhas.
use rand::prelude::*;

/// Definição dos caracteres válidos.
/// # Tipos
/// * `Number` - caracteres de dígitos entre 0 a 9
/// * `LowerCase` - caracteres minúsculos de a a z
/// * `UpperCase` - caracteres maiúsculos de A a Z
/// * `SpecialChar` - caracteres especiais
enum ValidCharacters {
    Number(char),
    LowerCase(char),
    UpperCase(char),
    SpecialChar(char)
}

/// Estrutura de uma senha válida.
/// 
/// # Atributos
/// * `has_number` - booleano para verificar se há `Number`
/// * `has_upper_case_char` - booleano para verificar se há `UpperCase`
/// * `has_lower_case_char` - booleano para verificar se há `LowerCase`
/// * `has_special_char` - booleano para verificar se há `SpecialChar`
struct IsPassValid {
    has_number: bool,
    has_upper_case_char: bool,
    has_lower_case_char: bool,
    has_special_char: bool,
}

impl IsPassValid {
    /// Retorna verdadeiro se tiver todos os atributos da estrutura verdadeiros.
    fn is_valid(&self) -> bool {
        self.has_lower_case_char && self.has_number && self.has_special_char && self.has_upper_case_char
    }
}

/// Função de sugestão de senha forte.
/// Cria um novo vetor com as propriedades de [ValidCharacters](ValidCharacters).
/// Gera uma senha de 12 caracteres com caracteres aleatórios.
/// Valida a senha, para que haja todos os tipos especificados em [ValidCharacters](ValidCharacters).
/// Retorna a senha como um vetor de caracteres ([String](String)).
pub fn suggest_strong_password() -> String {
    
    let mut password: Vec<ValidCharacters> = Vec::new();

    for _ in 0..12 {
        let option: i32 = thread_rng().gen_range(0..=3);
        match option {
            0 => password.push(get_number()),
            1 => password.push(get_lower_char()),
            2 => password.push(get_special_char()),
            _ => password.push(get_upper_char())
        }
    }
    let password = validate_password(password);
    unwrap_password(password)
}

/// Função para validação de senha.
/// Verifica se a senha tem todos os tipos de caracteres necessários.
/// Retorna um vetor de [ValidCharacters](ValidCharacters).
/// 
/// # Parâmetros
/// * `password` - variável mutável de um vetor de [ValidCharacters](ValidCharacters).
fn validate_password(mut password: Vec<ValidCharacters>) -> Vec<ValidCharacters> {
    loop {
        let is_password_valid = check_if_password_is_valid(&password);
        if is_password_valid.is_valid() { break; }

        if !is_password_valid.has_number {
            _ = replace(&mut password[thread_rng().gen_range(0..12)], get_number());
        }
    
        if !is_password_valid.has_lower_case_char {
            _ = replace(&mut password[thread_rng().gen_range(0..12)], get_lower_char());
        }
    
        if !is_password_valid.has_special_char {
            _ = replace(&mut password[thread_rng().gen_range(0..12)], get_special_char());
        }
    
        if !is_password_valid.has_upper_case_char {
            _ = replace(&mut password[thread_rng().gen_range(0..12)], get_upper_char());
        }
    }
    password
}

/// Função para verificação de senha válida.
/// Checa cada letra da senha para verificar se há os atributos necessários para a senha ser válida.
/// 
/// # Parâmetros
/// * `password` - variável mutável de um vetor de [ValidCharacters](ValidCharacters).
fn check_if_password_is_valid(password: &Vec<ValidCharacters>) -> IsPassValid {
    let mut res = IsPassValid {
        has_lower_case_char: false,
        has_number: false,
        has_special_char: false,
        has_upper_case_char: false
    };

    for char in password {
        match char {
            ValidCharacters::Number(_) => res.has_number = true,
            ValidCharacters::LowerCase(_) => res.has_lower_case_char = true,
            ValidCharacters::SpecialChar(_) => res.has_special_char = true,
            ValidCharacters::UpperCase(_) => res.has_upper_case_char = true
        }
    }

    res
}

/// Função para transformar a senha de um vetor de [ValidCharacters](ValidCharacters) para [String](String).
/// 
/// # Parâmetros
/// * `password` - variável mutável de um vetor de [ValidCharacters](ValidCharacters).
fn unwrap_password(password: Vec<ValidCharacters>) -> String {
    let mut char_vec = Vec::new();
    for character in password {
        let char = match character {
            ValidCharacters::LowerCase(a) => a,
            ValidCharacters::Number(a) => a,
            ValidCharacters::SpecialChar(a) => a,
            ValidCharacters::UpperCase(a) => a
        };
        char_vec.push(char);
    }
    char_vec.into_iter().collect()
}

/// Função para gerar um caractere de número aleatório entre 0 e 9.
fn get_number() -> ValidCharacters {
    ValidCharacters::Number(thread_rng().gen_range('0'..='9'))
}

/// Função para gerar um caractere especial aleatório.
fn get_special_char() -> ValidCharacters {
    // Inclui todos os caracteres especiais entre '!' e '~'
    ValidCharacters::SpecialChar(thread_rng().gen_range('!'..='~'))
}

/// Função para gerar um caractere maiúsculo aleatório entre A e Z.
fn get_upper_char() -> ValidCharacters {
    ValidCharacters::UpperCase(thread_rng().gen_range('A'..='Z'))
}

/// Função para gerar um caractere minúsculo aleatório entre a e z
fn get_lower_char() -> ValidCharacters {
    ValidCharacters::LowerCase(thread_rng().gen_range('a'..='z'))
}