use tokio::net::TcpStream;
use futures::{StreamExt, SinkExt};
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
use std::sync::Arc;
use tokio_postgres::{Client};
use crate::{message_repository::{self, MessageRepository}};

pub async fn handle_message(db_client: Arc<Client>, ws_stream: WebSocketStream<TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    let (mut write, mut read) = ws_stream.split();
    while let Some(message) = read.next().await {
        match message? {
            Message::Text(text) => {
                let db_client_clone = db_client.clone();
                let message_repository = message_repository::MessageRepository::new(db_client_clone);
                message_repository.save_chat_message(&text).await?;
                write.send(Message::Text(format!("Received: {}", text))).await?;
            },
            _ => (),
        }
    }

    Ok(())
}
