use std::future;
mod gateway;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    gateway::gateway::connect().await;
}
