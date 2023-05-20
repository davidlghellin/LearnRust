use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use learn_016_actix_web_003::{
    config::{read_config, Config},
    models::Status,
};
use log::info;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Activamos logs");

    let configuracion: Config = read_config();
    info!(
        "Estamos en el {:?}:{:?}",
        configuracion.host, configuracion.puerto
    );

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
