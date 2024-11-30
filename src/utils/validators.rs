use chrono::{Local, NaiveDateTime};

// Validador para título: não vazio e no máximo 50 caracteres
pub fn validate_title(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else if trimmed.len() > 50 {
        None
    } else {
        Some(trimmed.to_string())
    }
}

// Validador para hora no formato HH:MM
pub fn validate_time(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() == 2 {
        if let (Ok(h), Ok(m)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
            if h < 24 && m < 60 {
                return Some(input.to_string());
            }
        }
    }
    None
}

// Validador para data: não pode ser no passado
pub fn validate_date(input: &str) -> Option<NaiveDateTime> {
    if let Ok(parsed_date) = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", input), "%d-%m-%Y %H:%M:%S") {
        let now = Local::now().naive_local();
        if parsed_date >= now {
            return Some(parsed_date);
        }
    }
    None
}

// Valida a duração, garantindo que seja um número inteiro não negativo.
pub fn validate_duration(input: &str) -> Option<i32> {
    match input.trim().parse::<i32>() {
        Ok(value) if value >= 0 => Some(value),
        _ => None,
    }
}
