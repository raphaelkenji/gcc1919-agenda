use std::io::{self, Write};
use crate::models::{appointments::Prioridade, contacts::ContactCategory};

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

// Validador para categorias
pub fn parse_category(input: &str) -> Option<ContactCategory> {
    match input.to_lowercase().as_str() {
        "família" => Some(ContactCategory::Family),
        "amigo" => Some(ContactCategory::Friend),
        "trabalho" => Some(ContactCategory::Work),
        "outro" => Some(ContactCategory::Other),
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

// Entrada de categoria
pub fn category() -> ContactCategory {
    read_validated_input(
        "Nova categoria: ", 
        "Categoria inválida. Por favor, insira 'Família', 'Amigo', 'Trabalho', 'Outro'.", 
        parse_category)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number::<i32>("42"), Some(42));
        assert_eq!(parse_number::<f64>("3.14"), Some(3.14));
        assert_eq!(parse_number::<i32>("abc"), None);
    }

    #[test]
    fn test_parse_priority() {
        assert_eq!(parse_priority("baixa"), Some(Prioridade::Baixa));
        assert_eq!(parse_priority("media"), Some(Prioridade::Media));
        assert_eq!(parse_priority("alta"), Some(Prioridade::Alta));
        assert_eq!(parse_priority("unknown"), None);
    }

    #[test]
    fn test_parse_category() {
        assert_eq!(parse_category("família"), Some(ContactCategory::Family));
        assert_eq!(parse_category("amigo"), Some(ContactCategory::Friend));
        assert_eq!(parse_category("trabalho"), Some(ContactCategory::Work));
        assert_eq!(parse_category("outro"), Some(ContactCategory::Other));
        assert_eq!(parse_category("unknown"), None);
    }
}
