use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder, web::Json};
mod models;
use validator::Validate;
use crate::models::pizza::BuyPizzaRequest;


#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas disponibles")
}

#[post("/comprarpizza")]
async fn comprar_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    
    let is_valid = body.validate();
    match is_valid {
        Ok(_)=>{
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Comprando la pizza {pizza_name}"))
        },
        Err(_) => HttpResponse::Ok().body("El nombre de la pizza es requerido")
    }


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
