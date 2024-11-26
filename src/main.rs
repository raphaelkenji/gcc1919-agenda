mod utils;
mod services;
mod database;

use utils::input;
use services::{appointments, contacts};

#[tokio::main]
async fn main() {

    // Conecta com o banco de dados
    let _db = database::connect().await.unwrap();
    loop {
        println!("Selecione uma opção:");
        println!("1. Gerenciar compromissos");
        println!("2. Gerenciar contatos");
        println!("3. Sair");

        match input::number::<u8>("Opção: ") {
            1 => appointments::menu().await,
            2 => contacts::menu().await,
            3 => break,
            _ => println!("Opção inválida."),
        }
    }
}