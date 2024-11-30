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

// Valida a duração, garantindo que seja um número inteiro não negativo.
pub fn validate_duration(input: &str) -> Option<i32> {
    match input.trim().parse::<i32>() {
        Ok(value) if value >= 0 => Some(value),
        _ => None,
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
    fn test_validate_duration() {
        assert_eq!(validate_duration("30"), Some(30));
        assert_eq!(validate_duration("0"), Some(0));
        assert_eq!(validate_duration("-1"), None);
        assert_eq!(validate_duration("invalid"), None);
    }
}
