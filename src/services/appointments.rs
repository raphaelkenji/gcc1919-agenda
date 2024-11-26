
pub async fn menu() {

    loop {
        println!("Selecione uma opção:");
        println!("1. Adicionar compromisso");
        println!("2. Listar compromissos");
        println!("3. Atualizar compromisso");
        println!("4. Deletar compromisso");
        println!("5. Sair");
        
        match crate::utils::input::number::<u8>("Opção: ") {
            1 => add().await,
            2 => list().await,
            3 => update().await,
            4 => delete().await,
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

async fn add() {
    println!("Adicionando compromisso...");
}

async fn list() {
    println!("Listando compromissos...");
}

async fn update() {
    println!("Atualizando compromisso...");
}

async fn delete() {
    println!("Deletando compromisso...");
}