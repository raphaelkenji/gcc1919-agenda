use crate::dao::appointments_dao::AppointmentsDAO;
use crate::models::appointments::Appointments;
use crate::utils::input::get_optional_input;
use crate::utils::input::get_priority_input;
use crate::utils::input::string;
use crate::utils::input::number;
use crate::utils::pprint::{chose_appointments_from_table, print_appointments_table};

pub async fn menu(_db: &mongodb::Database) {
    loop {
        println!("Selecione uma opção:");
        println!("1. Adicionar compromisso");
        println!("2. Listar compromissos");
        println!("3. Atualizar compromisso");
        println!("4. Deletar compromisso");
        println!("5. Sair");

        match number("Opção: ") {
            1 => add(&_db).await,
            2 => list(&_db).await,
            3 => update(&_db).await,
            4 => delete(&_db).await,
            5 => break,
            _ => println!("Opção inválida."),
        }
    }
}

async fn add(_db: &mongodb::Database) {
    let titulo = string("Título: ");
    let descricao = string("Descrição: ");
    let data = string("Data (YYYY-MM-DD): ");
    let hora = string("Hora (HH:MM): ");
    let prioridade = get_priority_input();
    let mock_appointment = Appointments::new(titulo, data, hora, descricao, prioridade);

    println!("Adicionando compromisso...");
    let dao = AppointmentsDAO::new(_db).unwrap();
    dao.create(mock_appointment).await.unwrap();
}

async fn list(_db: &mongodb::Database) {
    let dao = AppointmentsDAO::new(_db).unwrap();
    let result = dao.read_all().await.unwrap();
    if result.is_empty() {
        println!("Não há compromissos");
    }
    print_appointments_table(&result);
}

async fn update(_db: &mongodb::Database) {
    let dao = AppointmentsDAO::new(_db).unwrap();
    let appointments = dao.read_all().await.unwrap();

    if appointments.is_empty() {
        println!("Não há compromissos");
        return;
    }

    let index = chose_appointments_from_table(&appointments);
    let selected_appointment = &appointments[index - 1];
    println!("\nCompromisso selecionado:");
    print_appointments_table(vec![selected_appointment]);

    println!("Digite um novo valor ou deixe em branco para manter o atual (exceto para prioridade):");
    let titulo = get_optional_input("Novo título: ");
    let descricao = get_optional_input("Nova descrição: ");
    let data = get_optional_input("Nova data (YYYY-MM-DD): ");
    let hora = get_optional_input("Nova hora (HH:MM): ");
    let prioridade = get_priority_input();

    let updated_appointment = Appointments::update_appointment(
        selected_appointment,
        titulo,
        data,
        hora,
        descricao,
        prioridade,
    );

    print_appointments_table(vec![&updated_appointment]);

    dao.update(updated_appointment).await.unwrap();
    println!("Compromisso atualizado com sucesso.");
}

async fn delete(_db: &mongodb::Database) {
    let dao = AppointmentsDAO::new(_db).unwrap();
    let appointments = dao.read_all().await.unwrap();

    if appointments.is_empty() {
        println!("Não há compromissos");
        return;
    }

    let index = chose_appointments_from_table(&appointments);
    let selected_appointment = &appointments[index - 1];

    dao.delete(&selected_appointment).await.unwrap();
    println!("Compromisso deletado com sucesso.");
}
