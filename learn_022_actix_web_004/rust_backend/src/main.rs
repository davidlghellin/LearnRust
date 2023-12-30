use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder, web::Json};
mod models;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas disponibles")
}

#[post("/comprarpizza")]
async fn comprar_pizza() -> impl Responder {

    HttpResponse::Ok().body("Comprando una pizza")
}

#[patch("/actualizarpizza/{uuid}")]
async fn actualizar_pizza() -> impl Responder {
    HttpResponse::Ok().body("Actualizando una pizza")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(comprar_pizza)
            .service(actualizar_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
