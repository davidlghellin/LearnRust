use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use prost::Message;
use proto_messages::ActorMessage;

fn main() {
    let server_ip = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let address = format!("{server_ip}:7878");

    println!("Client: Connecting to {address}");

    let mut stream = TcpStream::connect(&address).expect("Failed to connect to server");

    for i in 0..5 {
        println!("Client: Sending Ping...");

        let ping_msg = ActorMessage {
            command: "Ping".to_string(),
            payload: format!("Ping #{i}"),
        };

        let mut buf = Vec::new();
        ping_msg.encode(&mut buf).unwrap();

        stream.write_all(&buf).unwrap();

        let mut response_buf = [0; 512];
        let n = stream.read(&mut response_buf).unwrap();

        match ActorMessage::decode(&response_buf[..n]) {
            Ok(response_msg) => {
                println!(
                    "Client: Received command='{}', payload='{}'",
                    response_msg.command, response_msg.payload
                );
            }
            Err(e) => {
                eprintln!("Failed to decode response: {e:?}");
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    println!("Client: Sending Stop...");

    let stop_msg = ActorMessage {
        command: "Stop".to_string(),
        payload: String::new(),
    };

    let mut stop_buf = Vec::new();
    stop_msg.encode(&mut stop_buf).unwrap();
    stream.write_all(&stop_buf).unwrap();
}
