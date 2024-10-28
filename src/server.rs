use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use crossbeam::channel::unbounded;
use log::info;
use crate::command::Command;
use crate::resp::Value;

pub fn listen(host: &str, port: u16) {
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(address).expect("Failed to bind to address");

    let (tx, rx) = unbounded();

    let incoming_thread = thread::spawn( move || {
        for incoming in listener.incoming() {
            if let Ok(connection) = incoming {
                tx.send(connection)
                    .expect("Failed to send connection");
            }
        }
    });

    let handler_thread = thread::spawn( move || {
        loop {
            if let Ok(connection) = rx.recv() {
                handler(connection);
            }
        }
    });

    incoming_thread.join()
        .expect("Failed to join incoming thread");
    handler_thread.join()
        .expect("Failed to join handler thread");

}

fn handler(mut connection: TcpStream) {
    info!("Accepted connection from {}", connection.peer_addr().unwrap());

    loop {
        let mut buffer = [0; 1024];

        let length = connection.read(&mut buffer)
            .expect("Failed to read from stream");

        if length == 0 {
            break;
        }

        let reader = BufReader::new(&buffer[..length]);
        let value = match Value::read(reader) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Failed to read value: {}", e);
                continue;
            }
        };

        let command = Command::from(value);

        let response = command.run();

        response.write(&mut connection)
            .expect("Failed to write to stream");
    }
}