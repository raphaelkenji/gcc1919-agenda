use mongodb::Collection;
use crate::models::appointments::Appointments;
use crate::services::database::connect;

pub struct AppointmentsDAO {
    collection: Collection<Appointments>,
}

impl AppointmentsDAO {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database = connect().await?;

        let collection = database.collection::<Appointments>("appointments");
        Ok(Self { collection })
    }

    pub async fn create(&self, appointments: Appointments) -> Result<(), Box<dyn std::error::Error>> {
        self.collection.insert_one(appointments).await?;
        Ok(())
    }
}