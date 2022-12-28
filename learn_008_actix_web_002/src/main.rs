mod repository;
mod user;

use std::sync::atomic::AtomicU16;
use std::sync::Arc;

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use actix_web::middleware::Logger;
use log::info;
use repository::MemoryRepository;
use uuid::Uuid;

use crate::repository::RepositoryInjector;

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

async fn get_user(user_id: web::Path<String>, repo: web::Data<RepositoryInjector>) -> HttpResponse {
    if let Ok(parsed_user_id) = Uuid::parse_str(&user_id) {
        match repo.get_user(&parsed_user_id) {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().body("Not found"),
        }
    } else {
        HttpResponse::BadRequest().body("Invalid UUID")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init env vars
    dotenv::dotenv().ok();
    // building address
    let puerto: u16 = std::env::var("PORT")
        .expect("Puerto no definido")
        .parse()
        .expect("No puedo convertir, puerto mal definido");
    let host = std::env::var("HOST").expect("Puerto no definido");
    let address = format!("{}:{}", host, puerto);

    // Añadimos los logs
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Activamos logs");

    let repo = RepositoryInjector::new(MemoryRepository::default());
    let repo = web::Data::new(repo);

    let thread_counter: Arc<AtomicU16> = Arc::new(AtomicU16::new(1));
    HttpServer::new(move || {
        // en esta clojure compartirá todo lo que le pongamos
        let thread_index = thread_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        info!("Iniciando el thread: {}", thread_index);
        App::new()
            .wrap(Logger::default()) // añadir los logs
            .app_data(repo.clone())
            .app_data(thread_index)
            .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))
            .route("/health", web::get().to(HttpResponse::Ok))
            .route("/health2", web::get().to(healt))
            .route("/str", web::get().to(|| async { "Hola Rust {}" }))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!(
            "🔥🔥🔥 Couldn't start the server in port {}: {:?}",
            puerto, err
        )
    })
    .run()
    .await
}
