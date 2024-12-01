use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Appointments {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub titulo: String,
    pub data: bson::DateTime,
    pub hora: String,
    pub descricao: String,
    pub prioridade: Prioridade,
    pub duracao: i32,
}

impl Appointments {
    pub fn new(titulo: String, data: bson::DateTime, hora: String, descricao: String, prioridade: Prioridade, duracao: i32) -> Appointments {
        Appointments {
            id: bson::oid::ObjectId::new(),
            titulo,
            data,
            hora,
            descricao,
            prioridade,
            duracao
        }
    }
}

pub struct AppointmentBuilder {
    appointment: Appointments,
}

impl AppointmentBuilder {
    pub fn new(appointment: Appointments) -> Self {
        Self { appointment }
    }
    
    pub fn titulo(mut self, titulo: Option<String>) -> Self {
        if let Some(t) = titulo {
            self.appointment.titulo = t;
        }
        self
    }
    
    pub fn data(mut self, data: Option<bson::DateTime>) -> Self {
        if let Some(d) = data {
            self.appointment.data = d;
        }
        self
    }
    
    pub fn hora(mut self, hora: Option<String>) -> Self {
        if let Some(h) = hora {
            self.appointment.hora = h;
        }
        self
    }
    
    pub fn descricao(mut self, descricao: Option<String>) -> Self {
        if let Some(d) = descricao {
            self.appointment.descricao = d;
        }
        self
    }
    
    pub fn prioridade(mut self, prioridade: Option<Prioridade>) -> Self {
        if let Some(p) = prioridade {
            self.appointment.prioridade = p;
        }
        self
    }

    pub fn duracao(mut self, duracao: Option<i32>) -> Self {
        if let Some(d) = duracao {
            self.appointment.duracao = d;
        }
        self
    }
    
    pub fn build(self) -> Appointments {
        self.appointment
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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