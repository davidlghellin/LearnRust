use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Mensajes que el actor puede manejar
enum Message {
    Ping,
    Stop,
}

/// Actor que maneja los mensajes
fn pong_actor(rx: mpsc::Receiver<Message>) {
    for msg in rx {
        match msg {
            Message::Ping => {
                println!("Actor: Received Ping! Responding with Pong...");
            }
            Message::Stop => {
                println!("Actor: Received Stop. Shutting down.");
                break;
            }
        }
    }
}

fn main() {
    // Canal para enviar mensajes al actor
    let (tx, rx) = mpsc::channel();

    // Lanzamos el actor en un hilo separado
    let actor_handle = thread::spawn(move || {
        pong_actor(rx);
    });

    // Simula el cliente enviando pings
    for _ in 0..5 {
        println!("Main: Sending Ping...");
        tx.send(Message::Ping).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    // Ordenamos al actor que termine
    tx.send(Message::Stop).unwrap();

    // Esperamos a que el actor finalice
    actor_handle.join().unwrap();

    println!("Main: Actor has stopped. Exiting.");
}
