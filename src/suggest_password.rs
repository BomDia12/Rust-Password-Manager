use rand::prelude::*;

pub fn suggest_strong_password() {
    
    let mut senha: Vec<char> = Vec::with_capacity(10);
    let mut range = rand::thread_rng();

    for count in 0..10 {
        let option: i32 = range.gen_range(0..=3);
        match option {
            0 => senha.push(getNumber()),
            1 => senha.push(getLowerChar()),
            2 => senha.push(getSpecialChar()),
            3 => senha.push(getUpperChar()),
            _ => println!("Exception"), //fazer tratamento de excessão de verdade
        } 

    }
    let senhaFinal: String = senha.iter().collect();
    println!("{}",senhaFinal);
}

pub fn getNumber() -> char {
    thread_rng().gen_range('0'..='9')
}

pub fn getSpecialChar() -> char {
    // Inclui todos os caracteres especiais entre '!' e '~'
    thread_rng().gen_range('!'..='~')
}

pub fn getUpperChar() -> char {
    thread_rng().gen_range('A'..='Z')
}

pub fn getLowerChar() -> char {
    thread_rng().gen_range('a'..='z')
}