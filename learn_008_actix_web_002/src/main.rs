mod repository;
mod user;

use std::sync::atomic::AtomicU16;
use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use actix_web::middleware::Logger;
use log::info;
use repository::MemoryRepository;
use repository::Repository;
use uuid::Uuid;

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

async fn get_user(user_id: web::Path<Uuid>) -> HttpResponse {
    let memorepo = MemoryRepository::default();
    match memorepo.get_user(&user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let puerto: u16 = std::env::var("PORT")
        .expect("Puerto no definido")
        .parse()
        .expect("No puedo convertir, puerto mal definido");
    let host = std::env::var("HOST").expect("Puerto no definido");

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
            .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))
            .route("/", web::get().to(greet))
            .route("/health", web::get().to(HttpResponse::Ok))
            .route("/health2", web::get().to(healt))
            .route("/str", web::get().to(|| async { "Hola Rust {}" }))
            .route("/{name}", web::get().to(greet))
    })
    .bind((host, puerto))?
    .run()
    .await
}
