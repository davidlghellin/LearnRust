use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, num: Arc<Mutex<i32>>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Server Actor: Received '{}'", message.trim());

                if message.trim() == "Ping" {
                    // Bloqueamos el mutex para acceder y modificar el contador
                    let mut counter = num.lock().unwrap();
                    let response = format!("Pong {}\n", *counter);
                    stream.write_all(response.as_bytes()).unwrap();
                    *counter += 1;
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
