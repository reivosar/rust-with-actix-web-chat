use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::stream::StreamExt;
use futures::sink::SinkExt; 

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:3030";
    let listener: TcpListener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    let modified_text = format!("{} Received", text);
                    let new_msg = tokio_tungstenite::tungstenite::protocol::Message::text(modified_text);
                    write.send(new_msg).await.unwrap();
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
