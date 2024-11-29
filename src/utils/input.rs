use std::io::{self, Write};

use crate::models::appointments::Prioridade;
use chrono::NaiveDateTime;

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

// Lê uma data do terminal
pub fn date(prompt: &str) -> NaiveDateTime {
    loop {
        let input = string(prompt);
        let input_with_time = format!("{} 00:00:00", input);
        match NaiveDateTime::parse_from_str(&input_with_time, "%d-%m-%Y %H:%M:%S") {
            Ok(date) => return date,
            Err(_) => {
                println!("Data inválida. Por favor, insira uma data no formato DD-MM-YYYY.");
            }
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

// Função auxiliar para obter uma data opcional
pub fn get_optional_date(prompt: &str) -> Option<NaiveDateTime> {
    loop {
        let input = string(prompt);
        if input.trim().is_empty() {
            return None;
        }
        let input_with_time = format!("{} 00:00:00", input);
        match NaiveDateTime::parse_from_str(&input_with_time, "%d-%m-%Y %H:%M:%S") {
            Ok(date) => return Some(date),
            Err(_) => {
                println!("Data inválida. Por favor, insira uma data no formato DD-MM-YYYY.");
            }
        }
    }
}

// Função auxiliar para obter um numero opcional
pub fn get_optional_number<T: std::str::FromStr>(prompt: &str) -> Option<T> {
    Some(number(prompt))
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