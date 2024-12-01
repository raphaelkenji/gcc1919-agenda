use futures::TryStreamExt;
use mongodb::{bson, Collection};
use crate::models::appointments::Appointments;

pub struct AppointmentsDAO {
    collection: Collection<Appointments>,
}

impl AppointmentsDAO {
    pub fn new(database: &mongodb::Database) -> Result<Self, Box<dyn std::error::Error>> {

        let collection = database.collection::<Appointments>("appointments");
        Ok(Self { collection })
    }

    pub async fn create(&self, appointments: Appointments) -> Result<(), Box<dyn std::error::Error>> {
        self.collection.insert_one(appointments).await?;
        Ok(())
    }

    pub async fn read_all(&self) -> Result<Vec<Appointments>, Box<dyn std::error::Error>> {
        let filter = bson::doc! {};
        let mut cursor = self.collection.find(filter).await?;
        let mut result = Vec::<Appointments>::new();
        while let Some(doc) = cursor.try_next().await? {
            result.push(doc);
        }
        Ok(result)
    }
    
    pub async fn update(&self, appointment: Appointments) -> Result<(), Box<dyn std::error::Error>> {
        let filter = bson::doc! { "_id": appointment.id };
        let update = bson::to_document(&appointment)?;
        self.collection.update_one(filter, bson::doc! { "$set": update }).await?;

        Ok(())
    }

    pub async fn delete(&self, appointment: &Appointments) -> Result<(), Box<dyn std::error::Error>> {
        let filter = bson::doc! { "_id": appointment.id };
        self.collection.delete_one(filter).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn read_by_id(&self, appointment: &Appointments) -> Result<Option<Appointments>, Box<dyn std::error::Error>> {
        let filter = bson::doc! { "_id": appointment.id };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::database::connect;
    use crate::models::appointments::Prioridade;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_create_read_update_delete_appointment() {
        dotenv().ok();
        let db = connect().await.unwrap();
        let dao = AppointmentsDAO::new(&db).unwrap();

        let appointment = Appointments {
            id: bson::oid::ObjectId::new(),
            titulo: "Test Appointment".to_string(),
            descricao: "This is a test appointment".to_string(),
            data: bson::DateTime::from_chrono(chrono::Utc::now()),
            hora: "10:00".to_string(),
            prioridade: Prioridade::Alta,
            duracao: 20
        };

        dao.create(appointment.clone()).await.unwrap();

        let appointment_option = dao.read_by_id(&appointment).await.unwrap();
        assert!(appointment_option.is_some());
        let mut appointment = appointment_option.unwrap();
        assert_eq!(appointment.titulo, "Test Appointment");

        appointment.titulo = "Updated Test Appointment".to_string();
        dao.update(appointment.clone()).await.unwrap();

        let updated_appointment_option = dao.read_by_id(&appointment).await.unwrap();
        assert!(updated_appointment_option.is_some());
        let updated_appointment = updated_appointment_option.unwrap();
        assert_eq!(updated_appointment.titulo, "Updated Test Appointment");


        dao.delete(&appointment).await.unwrap();

        let deleted_appointment_option = dao.read_by_id(&appointment).await.unwrap();
        assert!(deleted_appointment_option.is_none());
    }
}
