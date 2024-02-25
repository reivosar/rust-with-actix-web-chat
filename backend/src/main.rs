use std::env;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};
use tokio_postgres::{NoTls, Error};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = "0.0.0.0:3030";
    let listener: TcpListener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

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

    while let Ok((stream, _)) = listener.accept().await {
        let client_clone = client.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, client_clone).await {
                eprintln!("handle connection error: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream, client: Arc<tokio_postgres::Client>) -> Result<(), Error> {
    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake occurred");

    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();

                    let stmt = client.prepare("INSERT INTO chat_messages (username, message) VALUES ('rust', $1)").await?;
                    client.execute(&stmt, &[&text]).await?;

                    let modified_text = format!("{} Received", text);
                    let new_msg = tokio_tungstenite::tungstenite::protocol::Message::text(modified_text);
                    write.send(new_msg).await.expect("Failed to send message");
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    Ok(())
}