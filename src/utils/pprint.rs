use crate::models::appointments::Appointments;
use std::borrow::Borrow;

pub fn print_appointments_table<T>(appointments: T)
where
    T: IntoIterator,
    T::Item: std::borrow::Borrow<Appointments>,
{
    println!(
        "{:<5} {:<20} {:<12} {:<8} {:<30} {:<10} {:<12}",
        "Nº", "Título", "Data", "Hora", "Descrição", "Duração", "Prioridade"
    );
    println!("{:-<100}", "");
    for (index, appointment) in appointments.into_iter().enumerate() {
        let appointment: &Appointments = appointment.borrow();
        println!(
            "{:<5} {:<20} {:<12} {:<8} {:<30} {:<10} {:<12}",
            index + 1,
            if appointment.titulo.len() > 17 {
                format!("{}...", &appointment.titulo[0..17])
            } else {
                appointment.titulo.clone()
            },
            appointment.data.to_chrono().format("%d-%m-%Y"),
            appointment.hora,
            if appointment.descricao.len() > 27 {
                format!("{}...", &appointment.descricao[0..27])
            } else {
                appointment.descricao.clone()
            },
            appointment.duracao,
            appointment.prioridade,
        );
    }
}

pub fn chose_appointments_from_table(appointments: &Vec<Appointments>) -> usize {
    print_appointments_table(appointments);
    let mut index =
        crate::utils::input::number::<usize>("Escolha o número do compormisso:");
    while index == 0 || index > appointments.len() {
        println!(
            "Número Inválido. Escolha um numero entre 1 and {}.",
            appointments.len()
        );
        index = crate::utils::input::number::<usize>("Escolha o número referente ao compormisso:");
    }
    index
}
