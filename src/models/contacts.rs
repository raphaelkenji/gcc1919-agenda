use mongodb::bson;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contacts {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub date_of_birth: bson::DateTime,
    pub age: i32,
    pub category: ContactCategory,
}

impl Contacts {
    pub fn new(name: String, email: String, phone: String, date_of_birth: bson::DateTime, age: i32, category: ContactCategory) -> Contacts {
        Contacts {
            id: bson::oid::ObjectId::new(),
            name,
            email,
            phone,
            date_of_birth,
            age,
            category,
        }
    }
}

pub struct ContactBuilder {
    contact: Contacts,
}

impl ContactBuilder {
    pub fn new(contact: Contacts) -> Self {
        Self { contact }
    }

    pub fn name(mut self, name: Option<String>) -> Self {
        if let Some(n) = name {
            self.contact.name = n;
        }
        self
    }

    pub fn email(mut self, email: Option<String>) -> Self {
        if let Some(e) = email {
            self.contact.email = e;
        }
        self
    }

    pub fn phone(mut self, phone: Option<String>) -> Self {
        if let Some(p) = phone {
            self.contact.phone = p;
        }
        self
    }

    pub fn date_of_birth(mut self, date_of_birth: Option<bson::DateTime>) -> Self {
        if let Some(d) = date_of_birth {
            self.contact.date_of_birth = d;
        }
        self
    }

    pub fn age(mut self, age: Option<i32>) -> Self {
        if let Some(a) = age {
            self.contact.age = a;
        }
        self
    }

    pub fn category(mut self, category: Option<ContactCategory>) -> Self {
        if let Some(c) = category {
            self.contact.category = c;
        }
        self
    }

    pub fn build(self) -> Contacts {
        self.contact
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ContactCategory {
    Family = 3,
    Friend = 2,
    Work = 1,
    Other = 0,
}

impl fmt::Display for ContactCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let categoria_str = match self {
            ContactCategory::Family => "Família",
            ContactCategory::Friend => "Amigo",
            ContactCategory::Work => "Trabalho",
            ContactCategory::Other => "Outro",
        };
        write!(f, "{}", categoria_str)
    }
}