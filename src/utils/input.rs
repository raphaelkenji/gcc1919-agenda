use std::io::{self, Write};

use crate::models::appointments::Prioridade;

// Lê uma string do terminal
pub fn string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

// Lê um número do terminal
pub fn number<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = string(prompt);
        match input.parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Insira um número válido."),
        }
    }
}

// Função auxiliar para obter um input opcional
pub fn get_optional_input(prompt: &str) -> Option<String> {
    let input = string(prompt);
    if input.trim().is_empty() {
        None
    } else {
        Some(input)
    }
}

// Função auxiliar para capturar e validar a prioridade
pub fn get_priority_input() -> Prioridade {
    loop {
        let input = string("Nova prioridade (Baixa, Media, Alta): ");
        match input.trim() {
            "Baixa" => return Prioridade::Baixa,
            "Media" => return Prioridade::Media,
            "Alta" => return Prioridade::Alta,
            _ => println!("Prioridade inválida. Por favor, insira 'Baixa', 'Media' ou 'Alta'."),
        }
    }
}