use std::io;

use actix_web::{web, App, HttpServer, Responder};
use log::info;


async fn status() -> impl Responder {
    "{\"status\":  \"UP\"}"
}

#[actix_rt::main]
async fn main() -> io::Result<()>{

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Activamos logs");
    info!("Estamos en el 127.0.0.1:8080");

    HttpServer::new(
        || App::new().route("/", web::get().to(status))
    ).bind("127.0.0.1:8080")?
    .run()
    .await
}
