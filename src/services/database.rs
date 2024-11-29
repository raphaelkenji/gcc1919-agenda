use mongodb::{error::Result, Client, Database};

#[allow(dead_code)]
pub async fn connect() -> Result<Database> {
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI não foi definida");
    match Client::with_uri_str(&mongo_uri).await {
        Ok(client) => {
            let database = client.database("agenda");
            Ok(database)
        }
        Err(e) => {
            eprintln!("Erro ao conectar com o banco de dados: {}", e);
            std::process::exit(1);
        }
    }
}
