//! # Gerenciador de senhas CLI
//! 
//! Este projeto foi desenvolvido em Rust no contexto da disciplina Linguagens de Programção
//! ministrada pelo professor Marcelo Ladeira. Ele visa explorar a linguagem Rust que foi escolhida pela turma
//! como a Linguagem a ser usada no desenvolvimento dos projetos.
//! 
//! # Desenvolvedores
//! - Ana Paula
//! - Arthur Mota Furtado (200014935)
//! - Gabriel Cruz
//! - Nicolas
//! - Vanessa Paixão
//! 
//! # Características da Linguagem Rust
//! 
//! Rust foi uma linguagem de programação desenvolvida por Graydon Hoare, e lançada em 2015. Ela tem como domínio
//! principal da linguagem o desenvolvimento de sistemas, e é uma linguagem multi-paradigmas, com ferramentas que 
//! permitem que ela seja utilizada tanto como uma linguagem imperativa, orientada a objetos, ou funcional.
//! 
//! Durante o seu desenvolvimento, o principal foco que rust teve foi em seguraça de memória, com um compilador,
//! que força um conjunto de regras, visando uma maior segurança em código rust. Não só isso, mas o funcionamento
//! do Borrow-checker, uma inovação do Rust, que propõe uma nova forma de lidar com variáveis, permite que o
//! programador não lide diretamente com gerenciamento de memória, mas retira a necessidade de um Garbage collector,
//! algo que diminui a performance das aplicações de linguagens que a implementam.
//! 
//! Um outro foco que Rust teve, em seu desenvolvimento, é a criação de arquivos compilados pequenos, o que permite que
//! seja uma linguagem com grande aplicabilidade no setor de embarcados.
//! 
//! ## Avaliaçao da linguagem em relação aos critérios de avaliação
//! 
//! ### Legibilidade
//! Rust é uma linguagem muito estruturada, e, uma vez que o programador é familiar com o funcionamento
//! da linguagem, a forma como o compilador força o código a ser escrito é muito mais fácil de se entender
//! do que é possível em uma linguagem como C.
//! 
//! Um exmplo claro disso, é a forma como rust implementa mutabilidade. Por padrão, as variáveis em Rust são constantes
//! e tem que ter um tipo a mais (`mut`) para serem mutáveis. Dessa forma, o programador, a ler o código entendente
//! extamente quais variáveis serão mudadas ao longo do desenvolvimento e quais não serão.
//! 
//! Rust tem muitas ferramentas que facilitam a leitura de código, uma das principais é o seu robusto sistema de tipagem.
//! Em Rust, todos os tipos básicos são bem intuitivos, com os inteiros (`u8`, `i8`, `u16`, ...), por exemplo,
//! explicitando se o tipo é possui sinal, e quantos bits o tipo tem (algo mais intuitivo que `char`, `short`, `int`,
//! `long` e `long long`). Porém, as estruras de dados mais poderosas de Rust tem que ser Enums, e Structs. Embora
//! inicialmente sejam parecidas com as estruras similares em C, elas possuem algumas ferramentas que as tornam muito
//! mais úteis.
//! 
//! Quando se trata de Structs, em Rust, elas tem um funcionamento similar a Classes, em linguagem orientadas a objetos.
//! Por meio da keyword `impl` é possível implementar métodos nessas Structs, tanto métodos gerais, que  não necessitam de
//! uma instância da Struct, quanto métodos específicos, que utlizam, ou modificam o objeto que chamou a função. Além
//! disso, quando uma Struct é declarada em um módulo, ela pode especificar quais propriedades da Struct são publicas
//! e quais são privadas.
//! 
//! Em relação a Enums, Rust os suporta como uma das principais estruturas de dados da linguagem. Tendo isso em vista,
//! Rust permite que cada valor enumerado receber uma variável de um tipo diferente, o que facilita muito com lidar com,
//! por exemplo, endereços de ip, como visto no código abaixo:
//! 
//! ```
//! enum IpAddr {
//!     V4(u8, u8, u8, u8)
//!     V6(String),
//! }
//! ```
//! 
//! Além disso, Rust também permite a implementação de métodos novos para enums, assim como para Structs.
//! 
//! Por ultimo, mas não menos importante, devido a implentação de mutabilidade como um tipo em Rust, a assinatura das
//! funções informa muito mais o usuário, uma vez que explicita se a função modifica, ou não um parâmetro, o que evita
//! erros de efeitos colaterais quando usando um biblioteca externa.
//! 
//! ### Capacidade de Escrita
//! 
//! Uma característica de Rust que aumenta a habilidade de escrita é como ele lida com o análogo dele aos ponteiros de C.
//! Em rust, um referência também é escrita como `&var` sendo `var` a variável que será referenciada. Porém, ao usar
//! uma referência, o programador não precisa adicionar alguma informação adicionar (o `*` do C) para utilizar a
//! referência, basta colocar o parâmetro que será utilizado, o compilador automaticamente traduz a referência
//! para o valor que deve ser usado.
//! 

/// Módulo que implementa as interfaces que integragem com o usuário
pub mod interfaces;
pub mod types;
pub mod encryption;
pub mod persistency;
/// Módulo que cria uma senha aleatória para o usuário
pub mod suggest_password;

use interfaces::cli;
use types::Entry;

fn main() {
    let master_password = "Test Password";
    let mut data: Vec<Entry> = Vec::new();

    cli::login_menu(&master_password, &mut data);
}
