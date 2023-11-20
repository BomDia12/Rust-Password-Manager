use std::io::stdin;

use crate::{types::Entry, persistency::{save_data_to_disk, read_data_from_disk}, suggest_password::suggest_strong_password};

pub fn login_menu(master_password: &str, data: &mut Vec<Entry>) {
    clear_terminal();
    println!("Por favor insira a sua senha mestra!");
    
    let mut inserted_password = String::new();

    stdin().read_line(&mut inserted_password).expect("Por favor insira um valor válido");

    let inserted_password = inserted_password.trim();

    if inserted_password == master_password {
        read_data_from_disk(data);
        return main_menu(data);
    }

    login_menu(master_password, data);
}

pub fn main_menu(data: &mut Vec<Entry>) {

    clear_terminal();

    println!("Bem vindo ao seu sistema de gerenciamento de senhas");

    loop {
        println!("Selecione uma de nossas opções para usar o sistema");
        println!("1 - Navegue por suas senhas salvas");
        println!("2 - Salve uma nova senha no nosso sistema");
        println!("3 - Saia do nosso sistema");
    
        let mut option = String::new();
    
        stdin().read_line(&mut option).expect("Por favor insira um valor válido");
    
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                clear_terminal();
                println!("Insira um inteiro, por favor!");
                continue;
            }
        };

        match option {
            1 => return stored_passwords(data),
            2 => return new_password(data),
            3 => return,
            _ => {
                clear_terminal();
                println!("Favor escolher uma opção válida");
                continue;
            }
        }
    }
}

pub fn new_password(data: &mut Vec<Entry>) {
    let mut domain = String::new();
    let mut username = String::new();
    let mut password = String::new();

    clear_terminal();
    println!("Insira o Domínio da senha a ser salva");
    stdin().read_line(&mut domain).expect("Por favor insira um valor válido");

    clear_terminal();
    println!("Insira o usuário a ser salvo");
    stdin().read_line(&mut username).expect("Por favor insira um valor válido");

    clear_terminal();
    println!("Insira a senha a ser salva, deixe vazio para receber uma senha forte");
    stdin().read_line(&mut password).expect("Por favor insira um valor válido");

    let domain = domain.trim().to_string();
    let username = username.trim().to_string();
    let password = password.trim().to_string();

    let password = {
        if password.is_empty() {
            suggest_strong_password()
        } else {
            password
        }
    };

    let new_entry = Entry {
        domain,
        username,
        password
    };

    data.push(new_entry);
    
    save_data_to_disk(data);

    main_menu(data);
}

pub fn stored_passwords(data: &mut Vec<Entry>) {
    clear_terminal();
    print_saved_passwords(data);
    loop {
        println!("Selecione uma das opções:");
        println!("1 - Listar senhas salvas novamente");
        println!("2 - Deletar uma senha (requer o índice da senha)");
        println!("3 - Voltar");

        let mut option = String::new();
        stdin().read_line(&mut option).expect("Por favor insira um valor válido");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                clear_terminal();
                println!("Insira um inteiro, por favor!");
                continue;
            }
        };

        match option {
            1 => {
                print_saved_passwords(data);
                continue;
            },
            2 => {
                delete_password(data);
                continue;
            },
            3 => return main_menu(data),
            _ => {
                clear_terminal();
                println!("Favor inserir uma opção válida");
                continue;
            }
        }
    }
}

fn print_saved_passwords(data: &mut Vec<Entry>) {
    for (i, entry) in data.iter().enumerate() {
        println!("{i} - {}", entry.domain);
        println!("Usuário: {}", entry.username);
        println!("Senha: {}", entry.password);
    }
}

fn delete_password(data: &mut Vec<Entry>) {
    println!("Insira o índice da senha a ser deletada (-1) para cancelar a operação");

    loop {
        let mut index = String::new();
    
        stdin().read_line(&mut index).expect("Por favor insira um valor válido");
    
        let index: i32 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Favor inserir um valor inteiro");
                continue;
            }
        };

        match index {
            -1 => return,
            _ => {
                let index = match usize::try_from(index).ok() {
                    Some(num) => num,
                    None => {
                        println!("Favor inserir um valor válido");
                        continue;
                    }
                };
                if index >= data.len() {
                    println!("Favor inserir um valor válido");
                    continue;
                }
                data.remove(index);
                save_data_to_disk(data);
                break;
            }
        }
    }
    clear_terminal();
}

#[inline(always)]
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
