use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // Leer IP del entorno o usar 127.0.0.1 por defecto
    let server_ip = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let address = format!("{}:7878", server_ip);

    println!("Client: Connecting to {}", address);

    let mut stream = TcpStream::connect(&address).expect("Failed to connect to server");

    for _ in 0..5 {
        println!("Client: Sending Ping...");
        stream.write_all(b"Ping\n").unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("Client: Received '{}'", response.trim());

        thread::sleep(Duration::from_secs(1));
    }

    println!("Client: Sending Stop...");
    stream.write_all(b"Stop\n").unwrap();
}
