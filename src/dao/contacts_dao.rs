use futures::TryStreamExt;
use mongodb::{bson, Collection};
use crate::models::contacts::Contacts;

pub struct ContactsDAO {
    collection: Collection<Contacts>,
}

impl ContactsDAO {
    pub fn new(database: &mongodb::Database) -> Result<Self, Box<dyn std::error::Error>> {

        let collection = database.collection::<Contacts>("contacts");
        Ok(Self { collection })
    }

    pub async fn create(&self, contacts: Contacts) -> Result<(), Box<dyn std::error::Error>> {
        self.collection.insert_one(contacts).await?;
        Ok(())
    }

    pub async fn read_all(&self) -> Result<Vec<Contacts>, Box<dyn std::error::Error>> {
        let filter = bson::doc! {};
        let mut cursor = self.collection.find(filter).await?;
        let mut result = Vec::<Contacts>::new();
        while let Some(doc) = cursor.try_next().await? {
            result.push(doc);
        }
        Ok(result)
    }
    
    pub async fn update(&self, contact: Contacts) -> Result<(), Box<dyn std::error::Error>> {
        let filter = bson::doc! { "_id": contact.id };
        let update = bson::to_document(&contact)?;
        self.collection.update_one(filter, bson::doc! { "$set": update }).await?;

        Ok(())
    }

    pub async fn delete(&self, contact: &Contacts) -> Result<(), Box<dyn std::error::Error>> {
        let filter = bson::doc! { "_id": contact.id };
        self.collection.delete_one(filter).await?;
        Ok(())
    }
}