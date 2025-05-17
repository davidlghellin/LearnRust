use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Simula el "Actor" del servidor
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Server Actor: Received '{}'", message);

                if message.trim() == "Ping" {
                    stream.write_all(b"Pong\n").unwrap();
                } else if message.trim() == "Stop" {
                    println!("Server Actor: Stopping as requested.");
                    break;
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

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {:?}", e),
        }
    }
}
