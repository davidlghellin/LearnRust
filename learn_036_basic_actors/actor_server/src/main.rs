use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use prost::Message;
use proto_messages::ActorMessage;

fn handle_client(mut stream: TcpStream, num: Arc<Mutex<i32>>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(n) => {
                let data = &buffer[..n];
                match ActorMessage::decode(data) {
                    Ok(msg) => {
                        println!(
                            "Server Actor: Received command='{}', payload='{}'",
                            msg.command, msg.payload
                        );

                        if msg.command == "Ping" {
                            let mut counter = num.lock().unwrap();
                            let response_msg = ActorMessage {
                                command: "Pong".to_string(),
                                payload: counter.to_string(),
                            };

                            let mut response_buf = Vec::new();
                            response_msg.encode(&mut response_buf).unwrap();

                            stream.write_all(&response_buf).unwrap();
                            *counter += 1;
                        } else if msg.command == "Stop" {
                            println!("Server Actor: Stopping as requested.");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to decode Protobuf message: {:?}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {:?}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Failed to bind to address");
    println!("Server listening on port 7878...");

    let num = Arc::new(Mutex::new(0));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let num_clone = Arc::clone(&num);
                thread::spawn(move || handle_client(stream, num_clone));
            }
            Err(e) => eprintln!("Connection failed: {:?}", e),
        }
    }
}
