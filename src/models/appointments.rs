use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Appointments {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub titulo: String,
    pub data: String,
    pub hora: String,
    pub descricao: String,
    pub prioridade: Prioridade,
}

impl Appointments {
    pub fn new(titulo: String, data: String, hora: String, descricao: String, prioridade: Prioridade) -> Appointments {
        Appointments {
            id: bson::oid::ObjectId::new(),
            titulo,
            data,
            hora,
            descricao,
            prioridade,
        }
    }

    pub fn update_appointment(old_appointment: &Appointments, titulo: Option<String>, data: Option<String>, hora: Option<String>, descricao: Option<String>, prioridade: Prioridade) -> Appointments {
        Appointments {
            id: old_appointment.id.clone(),
            titulo: titulo.unwrap_or_else(|| old_appointment.titulo.clone()),
            data: data.unwrap_or_else(|| old_appointment.data.clone()),
            hora: hora.unwrap_or_else(|| old_appointment.hora.clone()),
            descricao: descricao.unwrap_or_else(|| old_appointment.descricao.clone()),
            prioridade,
            ..*old_appointment
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Prioridade {
    Alta = 3,
    Media = 2,
    Baixa = 1,
}

impl fmt::Display for Prioridade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prioridade_str = match self {
            Prioridade::Alta => "Alta",
            Prioridade::Media => "Media",
            Prioridade::Baixa => "Baixa",
        };
        write!(f, "{}", prioridade_str)
    }
}