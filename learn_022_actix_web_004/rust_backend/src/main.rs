use actix_web::web::Data;
use actix_web::{get, patch, post, web::Json, web::Path, App, HttpResponse, HttpServer, Responder};
mod db;
mod models;
use crate::db::Database;
use crate::models::pizza::{BuyPizzaRequest, UpdatePizzaURL};
use validator::Validate;

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    //HttpResponse::Ok().body("Pizzas disponibles")
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        Some(pizzas_encontradas) => HttpResponse::Ok().body(format!("{:?}",pizzas_encontradas)),
        None =>HttpResponse::Ok().body("eRRPOR"),
    }
}

#[post("/comprarpizza")]
async fn comprar_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("Comprando la pizza {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("El nombre de la pizza es requerido"),
    }
}

#[patch("/actualizarpizza/{uuid}")]
async fn actualizar_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Actualizando la pizza con uuid {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await.expect("error conectando a la bbdd");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(comprar_pizza)
            .service(actualizar_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
