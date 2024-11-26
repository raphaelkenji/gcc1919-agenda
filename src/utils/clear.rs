use std::process::Command;

// Limpa a tela do terminal
#[allow(dead_code)]
pub fn screen() {
    if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else if cfg!(unix) {
        Command::new("clear")
            .status()
            .unwrap();
    }
}