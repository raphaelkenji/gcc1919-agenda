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
        while let Some(doc) = cursor.try_next().await? {
            println!("{:#?}", doc);
        };


        let mut result = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            result.push(doc);
        }
        Ok(result)
    }

    // pub async fn retitle(&self, titulo: &str) -> Result<Option<Appointments>, Box<dyn std::error::Error>> {
    //     let filter = doc! { "titulo": titulo };
    //     let Aments = self.collection.find_one(filter, None).await?;
    //     Ok(Appointments)
    // }

    // pub async fn update(&self, titulo: &str, updated: Appointments) -> Result<(), Box<dyn std::error::Error>> {
    //     let filter = doc! { "titulo": titulo };
    //     let update_doc = doc! {
    //         "$set": bson::to_document(&updated)?
    //     };
    //     self.collection.update_one(filter, update_doc, None).await?;
    //     Ok(())
    // }

    // pub async fn delete(&self, titulo: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let filter = doc! { "titulo": titulo };
    //     self.collection.delete_one(filter, None).await?;
    //     Ok(())
    // }
}