use log::{error, info};
use redis::server;

#[tokio::main]
async fn main(){
    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 6379;

    env_logger::init();

    info!("Starting Redis server...");

    if let Err(e) = server::listen(HOST, PORT).await {
        error!("Failed to start server: {}", e);
    }
}