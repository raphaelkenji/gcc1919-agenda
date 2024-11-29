use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Appointments {
    pub titulo: String,
    pub data: String,
    pub hora: String,
    pub descricao: String,
    pub prioridade: Prioridade,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Prioridade {
    Alta = 3,
    Media = 2,
    Baixa = 1,
}