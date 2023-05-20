use std::io;

use actix_web::{web, App, HttpServer, Responder};

async fn status() -> impl Responder {
    "{\"status\":  \"UP\"}"
}

#[actix_rt::main]
async fn main() -> io::Result<()>{
    HttpServer::new(
        || App::new().route("/", web::get().to(status))
    ).bind("127.0.0.1:8080")?
    .run()
    .await
}
