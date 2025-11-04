use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let url: &str = "wss://ws.ifelse.io";
    println!("Connecting to {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket connection established");
}
