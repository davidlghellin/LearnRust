mod users;

use actix_web::web::ServiceConfig;
use actix_web::{get, web};
use serde::Deserialize;

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/v1").configure(users::service));
}

// alternativa a service de v1, pero solo de como pasar el uuid
#[derive(Deserialize)]
struct Info {
    name: String,
}

// extract `Info` from a path using serde
#[get("/v2/{name}")]
async fn service2(info: web::Path<Info>) -> String {
    format!("Welcome {}!", info.name)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    // este es el get anterior
    cfg.service(service2);

    // este es el get como en v1 pasando todo
    cfg.service(web::scope("/user2").route("{user_id}", web::get().to(users::get)));
}
