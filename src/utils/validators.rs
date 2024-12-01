use chrono::{Datelike, Local, NaiveDateTime, Utc};
use regex;

// Validador para nome: não vazio e no máximo 50 caracteres
pub fn validate_name(input: &str) -> Option<String> {
    let trimmed: &str = input.trim();
    if trimmed.is_empty() {
        None
        } else if trimmed.len() > 50 {
        None
        } else {
        Some(trimmed.to_string())
        }
    }

// Validador para e-mail: não vazio e deve ser um e-mail válido
pub fn validate_email(input: &str) -> Option<String> {
    let trimmed = input.trim();
    let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if trimmed.is_empty() {
    None
    } else if !email_regex.is_match(trimmed) {
    None
    } else {
    Some(trimmed.to_string())
    }
}

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
    if let Ok(parsed_date) =
        NaiveDateTime::parse_from_str(&format!("{} 00:00:00", input), "%d-%m-%Y %H:%M:%S")
    {
        let now = Local::now().naive_local();
        if parsed_date >= now {
            return Some(parsed_date);
        }
    }
    None
}

// Validador para data de nascimento: não pode ser no futuro
pub fn validate_birth_date(input: &str) -> Option<NaiveDateTime> {
    if let Ok(parsed_date) =
        NaiveDateTime::parse_from_str(&format!("{} 00:00:00", input), "%d-%m-%Y %H:%M:%S")
    {
        let now = Local::now().naive_local();
        if parsed_date <= now {
            return Some(parsed_date);
        }
    }
    None
}

pub fn calculate_age(date_of_birth: NaiveDateTime) -> i32 {
    let now = Utc::now().naive_utc();
    let mut age = now.year() - date_of_birth.year();

    // Se ainda não fez aniversário este ano, diminui 1
    if now.month() < date_of_birth.month() || (now.month() == date_of_birth.month() && now.day() < date_of_birth.day()) {
        age -= 1;
    }

    age
}

// Valida a duração, garantindo que seja um número inteiro não negativo.
pub fn validate_duration(input: &str) -> Option<i32> {
    match input.trim().parse::<i32>() {
        Ok(value) if value >= 0 => Some(value),
        _ => None,
    }
}

// Valida número de telefone, garantindo que seja um número de telefone válido.
pub fn validate_phone(input: &str) -> Option<String> {
    let trimmed = input.trim();
    let phone_regex = regex::Regex::new(r"^\+?[\d\s-]+$").unwrap();
    if trimmed.is_empty() {
        None
    } else if !phone_regex.is_match(trimmed) {
        None
    } else {
        Some(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_title() {
        assert_eq!(
            validate_title("Titulo"),
            Some("Titulo".to_string())
        );
        assert_eq!(validate_title(""), None);
        assert_eq!(validate_title(" "), None);
        assert_eq!(validate_title("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged."), None);
    }

    #[test]
    fn test_validate_time() {
        assert_eq!(validate_time("12:30"), Some("12:30".to_string()));
        assert_eq!(validate_time("23:59"), Some("23:59".to_string()));
        assert_eq!(validate_time("24:00"), None);
        assert_eq!(validate_time("12:60"), None);
        assert_eq!(validate_time("abcxrty"), None);
    }

    #[test]
    fn test_validate_date() {
        let future_date = (Local::now() + chrono::Duration::days(1))
            .format("%d-%m-%Y")
            .to_string();
        let past_date = (Local::now() - chrono::Duration::days(1))
            .format("%d-%m-%Y")
            .to_string();
        assert!(validate_date(&future_date).is_some());
        assert!(validate_date(&past_date).is_none());
        assert!(validate_date("invalid-date").is_none());
    }

    #[test]
    fn test_validate_birth_date() {
        let future_date = (Local::now() + chrono::Duration::days(1))
            .format("%d-%m-%Y")
            .to_string();
        let past_date = (Local::now() - chrono::Duration::days(1))
            .format("%d-%m-%Y")
            .to_string();
        assert!(validate_birth_date(&past_date).is_some());
        assert!(validate_birth_date(&future_date).is_none());
        assert!(validate_birth_date("invalid-date").is_none());
    }

    #[test]
    fn test_validate_duration() {
        assert_eq!(validate_duration("30"), Some(30));
        assert_eq!(validate_duration("0"), Some(0));
        assert_eq!(validate_duration("-1"), None);
        assert_eq!(validate_duration("invalid"), None);
    }

    #[test]
    fn test_validate_email() {
        assert_eq!(validate_email(""), None);
        assert_eq!(validate_email("invalid-email"), None);
        assert_eq!(validate_email("test@.com"), None);
        assert_eq!(validate_email("test@com"), None);
        assert_eq!(validate_email("test@domain.com"), Some("test@domain.com".to_string()));
        assert_eq!(validate_email("test@domain.co.uk"), Some("test@domain.co.uk".to_string()));
    }

    #[test]
    fn test_validate_phone() {
        assert_eq!(validate_phone(""), None);
        assert_eq!(validate_phone("invalid-phone"), None);
        assert_eq!(validate_phone("123456789"), Some("123456789".to_string()));
        assert_eq!(validate_phone("+123 456 789"), Some("+123 456 789".to_string()));
        assert_eq!(validate_phone("123-456-789"), Some("123-456-789".to_string()));
    }
}
