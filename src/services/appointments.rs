use crate::models::appointments::Appointments;
use crate::models::appointments::Prioridade;
use crate::dao::appointments_dao::AppointmentsDAO;

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
    let mock_appointment = Appointments {
        titulo: String::from("Reunião de Projeto"),
        descricao: String::from("Discussão sobre o andamento do projeto"),
        data: String::from("2023-10-10"),
        hora: String::from("10:00"),
        prioridade: Prioridade::Alta,
    };
    
    println!("Adicionando compromisso...");
    let dao = AppointmentsDAO::new().await.expect("Failed to create AppointmentsDAO");
    dao.create(mock_appointment).await.unwrap();
    
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