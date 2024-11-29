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
}