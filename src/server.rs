use crate::command::Command;
use crate::resp::Value;
use log::{debug, error, info};
use std::error::Error;
use std::io::{BufReader};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};


pub async fn listen(host: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&address).await?;

    info!("Listening on {}", address);

    loop {
        debug!("Waiting for incoming connections...");
        let (socket, socket_address) = listener.accept().await?;

        info!("Accepted connection from {}", socket_address);

        tokio::spawn(async move { handler(socket).await.expect("handler error") });
    }
}

async fn handler(mut connection: TcpStream) -> Result<(), Box<dyn Error>> {
    loop {
        let mut buffer = [0; 1024];

        let length = connection.read(&mut buffer).await?;

        if length == 0 {
            break;
        }

        info!("Received {} bytes", length);

        let reader = BufReader::new(&buffer[..length]);
        let value = Value::read(reader).unwrap_or_else(|e| {
            error!("Failed to read value: {}", e);
            Value::Error(e.to_string())
        });

        let command = Command::from(value);

        let response = command.run();

        let bytes = response.read_bytes(Vec::new());

        connection.write_all(bytes.as_slice()).await?;
    }

    Ok(())
}