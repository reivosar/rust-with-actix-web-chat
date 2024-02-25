use tokio_postgres::Error;
use std::sync::Arc;
use tokio_postgres::Client;

pub struct MessageRepository {
    client: Arc<Client>,
}

impl MessageRepository {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn save_chat_message(&self, message: &str) -> Result<(), Error> {
        let user = "rust";
        self.client.execute(
            "INSERT INTO chat_messages (username, message) VALUES ($1, $2)",
            &[&user, &message]
        ).await?;

        Ok(())
    }
}