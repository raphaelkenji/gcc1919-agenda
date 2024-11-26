
pub async fn menu() {

    loop {
        println!("Selecione uma opção:");
        println!("1. Adicionar contato");
        println!("2. Listar contatos");
        println!("3. Atualizar contato");
        println!("4. Deletar contato");
        println!("5. Sair");
        
        match crate::utils::input::number::<u8>("Opção: ") {
            1 => add().await,
            2 => list().await,
            3 => update().await,
            4 => delete().await,
            5 => {
                println!("Encerrando...");
                break;
            },
            _ => println!("Opção inválida."),
        }
    }
}

async fn add() {
    println!("Adicionando contato...");
}

async fn list() {
    println!("Listando contatos...");
}

async fn update() {
    println!("Atualizando contato...");
}

async fn delete() {
    println!("Deletando contato...");
}