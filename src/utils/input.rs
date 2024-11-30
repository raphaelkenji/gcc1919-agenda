use std::io::{self, Write};
use crate::models::appointments::Prioridade;

// Função genérica para ler entrada do terminal
pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

// Função genérica para ler e validar entrada obrigatória
pub fn read_validated_input<T, F>(prompt: &str, error_message: &str, validator: F) -> T
where
    F: Fn(&str) -> Option<T>,
{
    loop {
        if let Some(value) = validator(&read_input(prompt)) {
            return value;
        } else {
            println!("{}", error_message);
        }
    }
}

// Validador para números
pub fn parse_number<T: std::str::FromStr>(input: &str) -> Option<T> {
    input.parse::<T>().ok()
}

// Validador para prioridades
pub fn parse_priority(input: &str) -> Option<Prioridade> {
    match input.to_lowercase().as_str() {
        "baixa" => Some(Prioridade::Baixa),
        "media" => Some(Prioridade::Media),
        "alta" => Some(Prioridade::Alta),
        _ => None,
    }
}

// Entrada de número obrigatório
pub fn number<T: std::str::FromStr>(prompt: &str) -> T {
    read_validated_input(prompt, "Insira um número válido.", parse_number)
}

// Entrada de prioridade
pub fn priority() -> Prioridade {
    read_validated_input(
        "Nova prioridade (Baixa, Media, Alta): ",
        "Prioridade inválida. Por favor, insira 'Baixa', 'Media' ou 'Alta'.",
        parse_priority,
    )
}
