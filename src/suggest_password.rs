use rand::prelude::*;

pub fn suggest_strong_password() -> String {
    
    let mut senha: Vec<char> = Vec::with_capacity(10);
    let mut range = rand::thread_rng();

    for _ in 0..12 {
        let option: i32 = range.gen_range(0..=3);
        match option {
            0 => senha.push(get_number()),
            1 => senha.push(get_lower_char()),
            2 => senha.push(get_special_char()),
            3 => senha.push(get_upper_char()),
            _ => println!("Exception"), //fazer tratamento de excessão de verdade
        } 

    }
    let senha_final: String = senha.iter().collect();
    senha_final
}

pub fn get_number() -> char {
    thread_rng().gen_range('0'..='9')
}

pub fn get_special_char() -> char {
    // Inclui todos os caracteres especiais entre '!' e '~'
    thread_rng().gen_range('!'..='~')
}

pub fn get_upper_char() -> char {
    thread_rng().gen_range('A'..='Z')
}

pub fn get_lower_char() -> char {
    thread_rng().gen_range('a'..='z')
}