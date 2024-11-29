use crate::models::appointments::Appointments;
use std::borrow::Borrow;

pub fn print_appointments_table<T>(appointments: T)
where
    T: IntoIterator,
    T::Item: std::borrow::Borrow<Appointments>,
{
    println!(
        "{:<5} {:<20} {:<12} {:<8} {:<30} {:<10}",
        "Nº", "Título", "Data", "Hora", "Descrição", "Prioridade"
    );
    println!("{:-<90}", "");
    for (index, appointment) in appointments.into_iter().enumerate() {
        let appointment = appointment.borrow();
        println!(
            "{:<5} {:<20} {:<12} {:<8} {:<30} {:<10}",
            index + 1,
            appointment.titulo,
            appointment.data,
            appointment.hora,
            if appointment.descricao.len() > 27 {
                appointment.descricao.chars().take(27).collect::<String>() + "..."
            } else {
                appointment.descricao.clone()
            },
            appointment.prioridade
        );
    }
}

pub fn chose_appointments_from_table(appointments: &Vec<Appointments>) -> usize {
    print_appointments_table(appointments);
    let mut index =
        crate::utils::input::number::<usize>("Escolha o número do compormisso para atualizar:");
    while index == 0 || index > appointments.len() {
        println!(
            "Número Inválido. Escolha um numero entre 1 and {}.",
            appointments.len()
        );
        index = crate::utils::input::number::<usize>("Escolha o número referente ao compormisso:");
    }
    index
}
