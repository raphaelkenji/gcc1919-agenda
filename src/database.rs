use mongodb::{bson::Document, error::Result, Client, Database};
use futures::stream::TryStreamExt;

// Implementação da função de conexão com o banco de dados
#[allow(dead_code)]
pub async fn connect() -> Result<Database> {
    match Client::with_uri_str("mongodb://localhost:27017").await {
        Ok(client) => {
            let database = client.database("agenda");

            // Exemplo de consulta ao banco de dados (fetch all)
            let collection = database.collection::<Document>("appointments");
            let cursor = collection.find(Document::new()).await?;
            let appointments: Vec<_> = cursor.try_collect().await?;

            for appointment in appointments {
                println!("{:?}", appointment);
            }

            Ok(database)
        }
        Err(e) => {
            eprintln!("Erro ao conectar com o banco de dados: {}", e);
            std::process::exit(1);
        }
    }
}