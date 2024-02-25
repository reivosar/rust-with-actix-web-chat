use tokio_postgres::{Client, NoTls, Error};
use std::env;
use std::sync::Arc;

pub struct DbConnectionManager;

impl DbConnectionManager {
    pub async fn new_connection() -> Result<Arc<Client>, Error> {
        let connection_string = format!(
            "postgresql://{}:{}@{}:{}/{}",
            env::var("DB_USER").expect("DB_USER not set"),
            env::var("DB_PASSWORD").expect("DB_PASSWORD not set"),
            env::var("DB_HOST").expect("DB_HOST not set"),
            "5432", 
            env::var("DB_NAME").expect("DB_NAME not set")
        );
    
        println!("connection start: {}", connection_string);
        let (client, connection) =
            tokio_postgres::connect(&connection_string, NoTls).await?;

        let client = Arc::new(client);
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(client)
    }
}