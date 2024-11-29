use crate::dao::appointments_dao::AppointmentsDAO;
use crate::models::appointments::{AppointmentBuilder, Appointments};
use crate::utils::input::{
    date, get_optional_date, get_optional_input, get_optional_number, get_priority_input, number,
    string,
};
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
    let data = date("Data (DD-MM-AAAA): ");
    let hora = string("Hora (HH:MM): ");
    let prioridade = get_priority_input();
    let duracao = number("Duração (minutos): ");

    let mock_appointment = Appointments::new(
        titulo,
        bson::DateTime::from_chrono(data.and_utc()),
        hora,
        descricao,
        prioridade,
        duracao,
    );

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

    println!(
        "Digite um novo valor ou deixe em branco para manter o atual (exceto para prioridade):"
    );
    let appointment_builder = AppointmentBuilder::new((*selected_appointment).clone());

    let titulo = get_optional_input("Novo título: ");
    let descricao = get_optional_input("Nova descrição: ");
    let data = get_optional_date("Nova data (DD-MM-AAAA): ")
        .map(|d| bson::DateTime::from_chrono(d.and_utc()));
    let hora = get_optional_input("Nova hora (HH:MM): ");
    let prioridade = Some(get_priority_input());
    let duracao = get_optional_number::<i32>("Nova Duração (segundos): ");

    let new_appointment = appointment_builder
        .titulo(titulo)
        .descricao(descricao)
        .data(data)
        .hora(hora)
        .prioridade(prioridade)
        .duracao(duracao)
        .build();

    print_appointments_table(vec![&new_appointment]);

    dao.update(new_appointment).await.unwrap();
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
