use mongodb::{error::Result, Client, Database};

#[allow(dead_code)]
pub async fn connect() -> Result<Database> {
    match Client::with_uri_str("mongodb+srv://agenda:e8VDcmYwQeoTmRjT@cluster0.ejbv3.mongodb.net/").await {
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
