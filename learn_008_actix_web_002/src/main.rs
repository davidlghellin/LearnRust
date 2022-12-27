use std::sync::atomic::AtomicU16;
use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use actix_web::middleware::Logger;
use log::info;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("Mundo");
    format!("Hola {}!\n", &name)
}

async fn healt(_req: HttpRequest) -> impl Responder {
    "Hello world!"
        .customize()
        .with_status(StatusCode::OK)
        .insert_header(("x-hello", "world"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Añadimos los logs
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Activamos logs");

    let thread_counter: Arc<AtomicU16> = Arc::new(AtomicU16::new(1));
    // .route("/health", web::get().to(move ||{HttpResponse::Ok()))})
    // .route("/health", web::get().to(move || HttpResponse::Ok().insert_header(Header("name", index_thread.to_string())).finish()))
    HttpServer::new(move || {
        // en esta clojure compartirá todo lo que le pongamos
        info!(
            "Iniciando el thread: {}",
            thread_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );
        //let index_thread = thread_counter.load(std::sync::atomic::Ordering::SeqCst);
        App::new()
            .wrap(Logger::default()) // añadir los logs
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(HttpResponse::Ok))
            .route("/health2", web::get().to(healt))
            .route("/str", web::get().to(|| async { "Hola Rust {}" }))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
