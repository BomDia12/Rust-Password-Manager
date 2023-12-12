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
//! Em termos mais gerais, não tem uma capacidade de escrita muito elevada. Como o principal foco de Rust é confiança e
//! segurança, o compilador aplica muitas regras que o programador deve seguir, se não o programa não compila. Um bom
//! exemplo disso é o borrow-checker, que deixa a passagem de parâmetros muito mais complicada em Rust.
//! 
//! Por outro lado, Rust possui um bom suporte para a abstração, com a adição de métodos a structs e enums sendo um bom
//! exemplo disso. Uma outra forma que Rust aumenta o suporte para a abstração é a facilidade com que você pode segregar
//! diferentes partes do código para que um arquivo só exporte as funções finais que ele implementa, não qualquer uma 
//! intermediaria, que seja para uso apenas interno.
//! 
//! Em termos de expressividade e sintaxe Rust é bem similar a C, não deviando muito nem para pior ou para melhor.
//! 
//! ### Confiabilidade
//! 
//! Confiabilidade é a área em que Rust mais se destaca, como o foco inteiro da linguagem é seguraça, muito trabalho
//! foi realizado para que a linguagem seja o mais confiável possivel.
//! 
//! Rust tem uma verificação de tipos estática, em tempo de compilação. Mas não só isso, Rust implementa com tipos
//! algumas coisas que outras linguagens deixariam a parte, como exceções e mutabilidade. A forma como Rust lida com
//! a checagem de Structs ou Enums que tenham os mesmos tipos internos, mas nomes diferentes é com a checagem de nomes,
//! o que também aumenta a confiabilidade na linguagem.
//! 
//! Tratamento de exceções. Rust é muito bom no quesito tratamento de exceções, como dito anteriormente, rust lida com
//! exceções, na maioria das vezes como um tipo, com funções que podem falhar retornando o seguinte Enum:
//! ```
//! enum Result<T, E> {
//!     Ok(T),
//!     Err(E),
//! }
//! ```
//! Dessa forma, o Usuário é obrigado a lidar com o erro de alguma forma antes de poder usar o resultado da função. Não
//! só isso, mas o Rust incentivo o tratamento de exceções e lidar com diferentes valores de uma variável com uma expressão
//! `match`, que, embora inicialmente pareça similar a um `switch-case` força o programador a lidar com todos os valores possíveis
//! para aquela variável, o que aumenta muito a confiabilidade.
//! 
//! Mais, o que é provavelmente a maior razão para a confiabilidade do Rust é o Borrow-checker. Esta é um estapa na compilação que
//! enforça um conjunto de regras que tem como objetivo minimizar os problemas que o programados pode ter com efeitos colaterais e
//! os valores dentro de uma função. O que o Borrow-checker enforça é que cada variável só pode ter um "dono", e, assim que este dono
//! acaba de ser executado, a variável é liberada da memória (é assim que Rust consegue não fazer uso de um Garbage collector). Além disso,
//! por padrão, apenas a função que é "dona" da variável pode modificar a variável. Nesse viés, para passarmos parâmetros em Rust, podemos
//! fazer de 3 formas diferentes.
//! 
//! Passagem por referência:
//! ```
//! fn fun(a: &i32) -> i32 {
//!     a + 10
//! }
//! 
//! fn main () {
//!     let a = 1;
//!     let b = fun(&a);
//!     println!("a : {}; b : {}", &a, &b);
//! }
//! ```
//! Na passagem por referência, a função que recebe a variável apenas a consegue ler, não edita-la, o que evita efeitos colaterais.
//! Uma variável pode ser referenciada várias vezes ao mesmo tempo.
//! 
//! Passagem por referência mutável:
//! ```
//! fn fun(a: &mut i32) {
//!     a += 10;
//! }
//! 
//! fn main () {
//!     let a = 1;
//!     fun(&mut a);
//!     println!("a : {};", &a);
//! }
//! ```
//! Na passagem por referência mutável, a função chamada pode modificar a variável passada, mas a função chamadora ainda é a dona
//! da variável. Como isso pode causar efeito colateral, apenas um função pode acessar uma variável por referência mutável por
//! vez, e não permite que a variável seja lida até que a função chamada resolva, para evitar efeitos colaterais negativos.
//! 
//! Passagem de propriedade:
//! ```
//! fn fun(a: i32) -> i32 {
//!     a += 10;
//! }
//! 
//! fn main () {
//!     let a = 1;
//!     let a = fun(a);
//!     println!("a : {};", &a);
//! }
//! ```
//! Quando um função é chamada por passagem de propriedade, a função chamada vira a dona do valor passado, dessa forma, a função
//! chamadora perde acesso aquela variável e não consegue mais utilizar aquela variável.
//! 
//! Com essas 3 formas de passagem de parâmetros Rust deixa problemas com valores inesperados de variáveis algo basicamente impossível
//! de acontecer.
//! 
//! ### Custo
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
