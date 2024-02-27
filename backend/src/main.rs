mod db_connection_manager;
mod message_handler;
mod message_repository;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use db_connection_manager::DbConnectionManager;
use message_handler::handle_message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:3030";
    let listener: TcpListener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);
    
    let db_client = DbConnectionManager::new_connection().await?;

    while let Ok((stream, _)) = listener.accept().await {
        let db_client_clone = db_client.clone();
        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    if let Err(e) = handle_message(db_client_clone, ws_stream).await {
                        println!("Error processing message: {}", e);
                    }
                },
                Err(e) => println!("Error during the websocket handshake: {}", e),
            }
        });
    }

    Ok(())
}
