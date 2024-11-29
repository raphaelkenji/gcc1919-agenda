mod utils;
mod services;
mod models;
mod dao;

use utils::input;
use services::{appointments, contacts};
use crate::services::database::connect;

#[tokio::main]
async fn main() {
    let _db = connect().await.unwrap();
    
    loop {
        println!("Selecione uma opção:");
        println!("1. Gerenciar compromissos");
        println!("2. Gerenciar contatos");
        println!("3. Sair");

        match input::number::<u8>("Opção: ") {
            1 => appointments::menu(&_db).await,
            2 => contacts::menu().await,
            3 => break,
            _ => println!("Opção inválida."),
        }
    }
}