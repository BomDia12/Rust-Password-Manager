use std::{io::stdin, path::Path};
use openssl::aes::AesKey;
use crate::{types::Entry, persistency::{save_data_to_disk, read_data_from_disk}, suggest_password::suggest_strong_password, encryption::generate_key};


/// Módulos necessários para o projeto
use crate::{
    types::Entry,
    persistency::{save_data_to_disk, read_data_from_disk},
    suggest_password::suggest_strong_password,
};

/// Função que implementa o menu de login do CLI.
///
/// Esta função solicita ao usuário a inserção da senha mestra e, se a senha fornecida
/// for a mesma de senha mestra fornecida como argumento, exibe o menu principal no terminal.
///
/// # Argumentos
///
/// * `master_password` - A senha mestra que o usuário deve fornecer para acessar o sistema.
/// * `data` - Um vetor mutável contendo as entradas do gerenciador de senhas.
///
/// # Exemplo
///
/// ```rust
/// let mut data = vec![]; // Inicializa o vetor de dados
/// login_menu("senha_mestra", &mut data);
/// ```
pub fn login_menu() {
    clear_terminal();
    println!("Por favor insira a sua senha mestra!");
    
    let mut inserted_password = String::new();

    stdin().read_line(&mut inserted_password).expect("Por favor insira um valor válido");

    let inserted_password = inserted_password.trim();

    let key = generate_key(inserted_password);
    match read_data_from_disk(&key) {
        Ok(data) => return main_menu(data, key),
        Err(_) => ()
    };

    login_menu();
}


/// Função que implementa o menu principal do sistema de gerenciamento de senhas.
///
/// Exibe um menu de opções para o usuário, permitindo a navegação por senhas salvas,
/// a adição de uma nova senha ou a saída do sistema.
/// O usuário é solicitado a inserir uma opção válida e ações correspondentes são realizadas.
///
/// # Argumentos
///
/// * `data` - Um vetor mutável contendo as entradas do gerenciador de senhas.
///
/// # Exemplo
///
/// ```rust
/// let mut data = vec![]; // Inicializa o vetor de dados
/// main_menu(&mut data);
/// ```
pub fn main_menu(data: Vec<Entry>, key: [u8; 32]) {

    clear_terminal(); // Função para limpar o terminal

    // Solicita ao usuário a inserção da senha mestra e, se a senha fornecida exibe o menu principal no terminal.
    println!("Bem vindo ao seu sistema de gerenciamento de senhas");

    // Loop principal do programa que exibe um menu de opções para o usuário.
    // O loop continua até que o usuário escolha sair do sistema.
    loop {

        // Exibe as opções disponíveis no menu principal
        println!("Selecione uma de nossas opções para usar o sistema");
        println!("1 - Navegue por suas senhas salvas");
        println!("2 - Salve uma nova senha no nosso sistema");
        println!("3 - Saia do nosso sistema");
    
        // Lê a opção escolhida pelo usuário
        let mut option = String::new();
        stdin().read_line(&mut option).expect("Por favor insira um valor válido");
    
        // Converte a opção para um número inteiro (u32)
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
             // Se a conversão falhar, limpa o terminal, exibe uma mensagem de como deve ser o valor inserido
            // e continua para a próxima iteração do loop
            Err(_) => {
                clear_terminal();
                println!("Insira um inteiro, por favor!");
                continue;
            }
        };

        // Utiliza uma expressão match para determinar a ação com base na opção escolhida pelo usuário.
        match option {
            // Se a opção for 1, chama a função para visualizar senhas armazenadas
            1 => return stored_passwords(data, key),
            // Se a opção for 2, chama a função para salvar uma nova senha
            2 => return new_password(data, key),
            // Se a opção for 3, sai do programa
            3 => return,
            // Se a opção não for nenhuma das anteriores, limpa o terminal, exibe uma mensagem de erro
            _ => {
                clear_terminal();
                println!("Favor escolher uma opção válida");
                continue;
            }
        }
    }
}

/// Função para adicionar uma nova senha ao sistema.
///
/// Solicita ao usuário as informações necessárias para criar uma nova entrada
/// (domínio, nome de usuário e senha) e a adiciona ao vetor de dados fornecido.
/// Caso o usuário opte por gerar uma senha forte, uma sugestão de senha forte é gerada.
///
/// # Argumentos
///
/// * `data` - Um vetor mutável contendo as entradas do gerenciador de senhas.
///
/// # Exemplo
///
/// ```rust
/// let mut data = vec![]; // Inicializa o vetor de dados
/// new_password(&mut data);
/// ```
pub fn new_password(data: Vec<Entry>, key: [u8; 32]) {
    let mut data = data;

    // Inicializa variáveis para armazenar o domínio, nome de usuário e senha
    let mut domain = String::new();
    let mut username = String::new();
    let mut password = String::new();

    // Solicita ao usuário o domínio/site da senha a ser salva
    clear_terminal();
    println!("Insira o Domínio da senha a ser salva");
    stdin().read_line(&mut domain).expect("Por favor insira um valor válido");

    // Solicita ao usuário o usuário a ser salvo
    clear_terminal();
    println!("Insira o usuário a ser salvo");
    stdin().read_line(&mut username).expect("Por favor insira um valor válido");

    // Solicita ao usuário a senha a ser salva, oferecendo a opção de gerar uma senha forte
    clear_terminal();
    println!("Insira a senha a ser salva, deixe vazio para receber uma senha forte");
    stdin().read_line(&mut password).expect("Por favor insira um valor válido");

    // Remove espaços em branco extras e converte para String
    let domain = domain.trim().to_string();
    let username = username.trim().to_string();
    let password = password.trim().to_string();

    // Se a senha for vazia, gera uma senha forte
    let password = {
        if password.is_empty() {
            suggest_strong_password()
        } else {
            password
        }
    };

    // Cria uma nova entrada com as informações fornecidas e a adiciona ao vetor de dados
    let new_entry = Entry {
        domain,
        username,
        password
    };

    data.push(new_entry);
    
    // Salva os dados 
    save_data_to_disk(&data, &key);

    // Retorna ao menu principal
    main_menu(data, key);
}


/// Função que implementa o menu para visualizar senhas salvas no sistema.
///
/// Exibe as senhas salvas, permitindo que o usuário escolha entre listar as senhas novamente,
/// deletar uma senha específica ou voltar ao menu principal.
///
/// # Argumentos
///
/// * `data` - Um vetor mutável contendo as entradas do gerenciador de senhas.
///
/// # Exemplo
///
/// ```rust
/// let mut data = vec![]; // Inicializa o vetor de dados
/// stored_passwords(&mut data);
/// ```
pub fn stored_passwords(data: Vec<Entry>, key: [u8; 32]) {

    let mut data = data;

    // Limpa o terminal antes de exibir o menu
    clear_terminal();
    // Exibe as senhas salvas
    print_saved_passwords(&data);

    // Loop principal do menu de senhas salvas
    loop {
        println!("Selecione uma das opções:");
        println!("1 - Listar senhas salvas novamente");
        println!("2 - Deletar uma senha (requer o índice da senha)");
        println!("3 - Voltar");

        // Lê a opção escolhida pelo usuário
        let mut option = String::new();
        stdin().read_line(&mut option).expect("Por favor insira um valor válido");

        // Converte a opção para um número inteiro 
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            // Se a conversão falhar, limpa o terminal, exibe uma mensagem de erro
            // e continua para a próxima iteração do loop
            Err(_) => {
                clear_terminal();
                println!("Insira um inteiro, por favor!");
                continue;
            }
        };

        // Utiliza uma expressão match para determinar a ação com base na opção escolhida pelo usuário.
        match option {

            // Se a opção for 1, exibe as senhas salvas novamente
            1 => {
                print_saved_passwords(&data);
                continue;
            },

            // Se a opção for 2, chama a função para deletar uma senha
            2 => {
                delete_password(&mut data, &key);
                continue;
            },

            // Se a opção for 3, retorna ao menu principal
            3 => return main_menu(data, key),

            // Se a opção não for nenhuma das anteriores, limpa o terminal, exibe uma mensagem de erro
            _ => {
                clear_terminal();
                println!("Favor inserir uma opção válida");
                continue;
            }
        }
    }
}


/// Função para imprimir as senhas salvas no sistema.
///
/// Exibe as informações de cada entrada (domínio, usuário e senha) armazenada
/// no vetor de dados fornecido.
///
/// # Argumentos
///
/// * `data` - Um vetor mutável contendo as entradas do gerenciador de senhas.
///
/// # Exemplo
///
/// ```rust
/// let mut data = vec![]; // Inicializa o vetor de dados
/// print_saved_passwords(&mut data);
/// ```
fn print_saved_passwords(data: &Vec<Entry>) {
    for (i, entry) in data.iter().enumerate() {

        // Imprime o índice da senha
        println!("{i} - {}", entry.domain);

        // Imprime o domínio, usuário e senha da entrada
        println!("Usuário: {}", entry.username);

        // Imprime a senha da entrada
        println!("Senha: {}", entry.password);
    }
}

/// Função para deletar uma senha do sistema com base no índice fornecido pelo usuário.
///
/// Solicita ao usuário o índice da senha a ser deletada e realiza a remoção da entrada
/// correspondente do vetor de dados. As alterações são então salvas no disco.
///
/// # Argumentos
///
/// * `data` - Um vetor mutável contendo as entradas do gerenciador de senhas.
///
/// # Exemplo
///
/// ```rust
/// let mut data = vec![]; // Inicializa o vetor de dados
/// delete_password(&mut data);
/// ```
fn delete_password(data: &mut Vec<Entry>, key: &[u8]) {

    // Solicita ao usuário o índice da senha a ser deletada
    println!("Insira o índice da senha a ser deletada (-1) para cancelar a operação");

    // Loop para ler o índice da senha a ser deletada
    loop {

        // Lê o índice fornecido pelo usuário
        let mut index = String::new();
    
        // Converte o índice para um número inteiro 
        stdin().read_line(&mut index).expect("Por favor insira um valor válido");
        
        // Converte o índice para um número inteiro 
        let index: i32 = match index.trim().parse() {
            Ok(num) => num,
            // Se a conversão falhar, limpa o terminal, exibe uma mensagem de erro
            Err(_) => {
                println!("Favor inserir um valor inteiro");
                continue;
            }
        };

        // Utiliza uma expressão match para determinar a ação com base no índice fornecido pelo usuário.
        match index {

            // Se o índice for -1, retorna ao menu de senhas salvas
            -1 => return,

            // Se o índice não for -1, remove a entrada correspondente do vetor de dados
            _ => {
                let index = match usize::try_from(index).ok() {

                    // Se a conversão falhar, limpa o terminal, exibe uma mensagem de erro
                    Some(num) => num,
                    None => {
                        println!("Favor inserir um valor válido");
                        continue;
                    }
                };

                // Verifica se o índice fornecido é válido
                if index >= data.len() {
                    println!("Favor inserir um valor válido");
                    continue;
                }

                // Remove a entrada correspondente do vetor de dados
                data.remove(index);

                // Salva os dados
                save_data_to_disk(&data, &key);
                break;
            }
        }
    }

    // Retorna ao menu de senhas salvas
    clear_terminal();
}

/// Função para limpar o terminal.
#[inline(always)] // Atributo para indicar que a função deve ser sempre inline
fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
