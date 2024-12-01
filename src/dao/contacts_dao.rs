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

    pub async fn read_by_id(&self, contact: &Contacts) -> Result<Option<Contacts>, Box<dyn std::error::Error>> {
        let filter = bson::doc! { "_id": contact.id };
        let result = self.collection.find_one(filter).await?;
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::database::connect;
    use crate::models::contacts::ContactCategory;
    use dotenvy::dotenv;


    #[tokio::test]
    async fn test_create_read_update_delete_contact() {
        dotenv().ok();
        let db = connect().await.unwrap();
        let dao = ContactsDAO::new(&db).unwrap();

        let contact = Contacts {
            id: bson::oid::ObjectId::new(),
            name: "Test Contact".to_string(),
            email: "test@example.com".to_string(),
            phone: "123456789".to_string(),
            age: 0,
            date_of_birth: bson::DateTime::from_chrono(chrono::Utc::now() - chrono::Duration::days(1)),
            category: ContactCategory::Family,
        };

        dao.create(contact.clone()).await.unwrap();

        let contact_option = dao.read_by_id(&contact).await.unwrap();
        assert!(contact_option.is_some());
        let mut contact = contact_option.unwrap();
        assert_eq!(contact.name, "Test Contact");

        contact.name = "Updated Test Contact".to_string();
        dao.update(contact.clone()).await.unwrap();

        let updated_contact_option = dao.read_by_id(&contact).await.unwrap();
        assert!(updated_contact_option.is_some());
        let updated_contact = updated_contact_option.unwrap();
        assert_eq!(updated_contact.name, "Updated Test Contact");

        dao.delete(&contact).await.unwrap();

        let deleted_contact_option = dao.read_by_id(&contact).await.unwrap();
        assert!(deleted_contact_option.is_none());
    }
}