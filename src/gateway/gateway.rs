use std::thread;
use std::time::Duration;

use futures_util::StreamExt;
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::Connector;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::tungstenite::protocol::Message;
use native_tls::TlsConnector;
use http::Request;

use futures_util::future;
use futures_util::pin_mut;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::gateway::opcodes::HELLO_10;

pub async fn connect() {
    let connector = TlsConnector::new().unwrap();
    let config = WebSocketConfig::default();
    let (websocket, response) = 
        connect_async_tls_with_config("wss://gateway.discord.gg/?encoding=json&v=9", Some(config), false, Some(Connector::NativeTls(connector))).await.expect("Failed to connect");

    println!("Code: {}", response.status());
    
    let (write, read) = websocket.split();


    let ws_to_stdout = {
        read.for_each(|message| async {
            handle(message);
        })
    };

    tokio::spawn(ws_to_stdout);

    thread::sleep(Duration::from_secs(200))

}

fn handle(message: Result<Message, Error>) {
    match message {
        Ok(msg) => {
            let data = msg.into_data();
            println!("{}", String::from_utf8_lossy(&data));
            let hello: HELLO_10 = serde_json::from_str(&String::from_utf8_lossy(&data)).expect("json parse error");
            println!("{:?}", hello);
        },
        Err(e) => {

        }
    }

}