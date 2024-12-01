
use crate::dao::contacts_dao::ContactsDAO;
use crate::models::contacts::{ContactBuilder, Contacts};
use crate::utils::input::*;
use crate::utils::pprint::{chose_contacts_from_table, print_contacts_table};
use crate::utils::validators::{validate_birth_date, validate_email, validate_name, validate_phone, calculate_age};

pub async fn menu(_db: &mongodb::Database) {

    loop {
        println!("Selecione uma opção:");
        println!("1. Adicionar contato");
        println!("2. Listar contatos");
        println!("3. Atualizar contato");
        println!("4. Deletar contato");
        println!("5. Sair");
        
        match crate::utils::input::number::<u8>("Opção: ") {
            1 => add(&_db).await,
            2 => list(&_db).await,
            3 => update(&_db).await,
            4 => delete(&_db).await,
            5 => {
                println!("Encerrando...");
                break;
            },
            _ => println!("Opção inválida."),
        }
    }
}

async fn add(_db: &mongodb::Database) {
    let contact = get_contact_details(None).await;
    let dao = ContactsDAO::new(_db).unwrap();
    dao.create(contact).await.unwrap();
    println!("Contato adicionado com sucesso.");
}

async fn list(_db: &mongodb::Database) {
    let dao = ContactsDAO::new(_db).unwrap();
    let result = dao.read_all().await.unwrap();
    if result.is_empty() {
        println!("Não há contatos");
    }
    print_contacts_table(&result);
}

async fn update(_db: &mongodb::Database) {
    let dao = ContactsDAO::new(_db).unwrap();
    let contacts = dao.read_all().await.unwrap();

    if contacts.is_empty() {
        println!("Não há contatos");
        return;
    }

    let index = chose_contacts_from_table(&contacts);
    let selected_contact = &contacts[index - 1];
    println!("\nContato selecionado: ");
    print_contacts_table(vec![selected_contact]);

    let contact = get_contact_details(Some(selected_contact)).await;
    dao.update(contact).await.unwrap();
    println!("Contato atualizado com sucesso.");
}

async fn delete(_db: &mongodb::Database) {
    let dao = ContactsDAO::new(_db).unwrap();
    let contacts = dao.read_all().await.unwrap();

    if contacts.is_empty() {
        println!("Não há contatos");
        return;
    }

    let index = chose_contacts_from_table(&contacts);
    let selected_contact = &contacts[index - 1];

    dao.delete(&selected_contact).await.unwrap();
    println!("Contato deletado com sucesso.");
}

async fn get_contact_details(existing_contact: Option<&Contacts>) -> Contacts {
    let name = read_validated_input(
        "Novo nome: ", 
        "O nome é obrigatório", 
        validate_name);
    let email = read_validated_input(
        "Novo email: ", 
        "E-mail inválido", 
        validate_email);
    let phone = read_validated_input(
        "Novo telefone: ", 
        "Telefone inválido", 
        validate_phone);
    let date_of_birth = read_validated_input(
        "Nova data de nascimento (dd-mm-aaaa): ", 
        "Data inválida", 
        validate_birth_date);
    let age = calculate_age(date_of_birth);
    let category = category();

    if let Some(existing) = existing_contact {
        ContactBuilder::new(existing.clone())
            .name(Some(name))
            .email(Some(email))
            .phone(Some(phone))
            .date_of_birth(Some(bson::DateTime::from_chrono(date_of_birth.and_utc())))
            .age(Some(age))
            .category(Some(category))
            .build()
    } else {
        Contacts::new(
            name,
            email, 
            phone, 
            bson::DateTime::from_chrono(date_of_birth.and_utc()), 
            age,
            category
        )
    }
}