use crate::dao::appointments_dao::AppointmentsDAO;
use crate::models::appointments::{AppointmentBuilder, Appointments};
use crate::utils::input::*;
use crate::utils::pprint::{chose_appointments_from_table, print_appointments_table};
use crate::utils::validators::{validate_date, validate_duration, validate_time, validate_title};

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
    let appointment = get_appointment_details(None).await;
    let dao = AppointmentsDAO::new(_db).unwrap();
    dao.create(appointment).await.unwrap();
    println!("Compromisso adicionado com sucesso.");
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
    println!("\nCompromisso selecionado: ");
    print_appointments_table(vec![selected_appointment]);

    let appointment = get_appointment_details(Some(selected_appointment)).await;
    dao.update(appointment).await.unwrap();
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

async fn get_appointment_details(existing_appointment: Option<&Appointments>) -> Appointments {
    let titulo = read_validated_input(
        "Novo título: ",
        "O título não pode ser vazio ou exceder 50 caracteres.",
        validate_title,
    );
    let descricao = read_input("Descrição: ");
    let data = read_validated_input(
        "Data (DD-MM-AAAA): ",
        "Data inválida. Certifique-se de que a data não seja no passado e esteja no formato DD-MM-AAAA.",
        validate_date,
    );
    let hora = read_validated_input(
        "Hora (HH:MM): ",
        "Hora inválida. Por favor, insira no formato HH:MM.",
        validate_time,
    );
    let prioridade = priority();
    let duracao = read_validated_input(
        "Duração (minutos): ",
        "Duração inválida. Deve ser um número inteiro não negativo.",
        validate_duration,
    );

    if let Some(existing) = existing_appointment {
        AppointmentBuilder::new(existing.clone())
            .titulo(Some(titulo))
            .descricao(Some(descricao))
            .data(Some(bson::DateTime::from_chrono(data.and_utc())))
            .hora(Some(hora))
            .prioridade(Some(prioridade))
            .duracao(Some(duracao))
            .build()
    } else {
        Appointments::new(
            titulo,
            bson::DateTime::from_chrono(data.and_utc()),
            hora,
            descricao,
            prioridade,
            duracao,
        )
    }
}