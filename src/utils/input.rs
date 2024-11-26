use std::io::{self, Write};

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