use actix_web::http::StatusCode;
use actix_web::{web, App, Error, HttpResponse, HttpServer};

async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().route("/", web::get().to(home)))
        .bind("127.0.0.1:8888")?
        .run()
        .await
}
