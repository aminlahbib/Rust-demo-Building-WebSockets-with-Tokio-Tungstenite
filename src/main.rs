use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;


#[tokio::main]
async fn main() {
    let url: &str = "wss://ws.ifelse.io";
    println!("Connecting to {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket connection established");

    let (mut write, mut read) = ws_stream.split();
    let msg = Message::text("Hello, echo server");
    write.send(msg).await.expect("Failed to send");
}
