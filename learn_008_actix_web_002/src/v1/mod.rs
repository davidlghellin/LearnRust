mod users;

use actix_web::web::ServiceConfig;
use actix_web::{get, web};
use serde::Deserialize;

use crate::repository::Repository;

pub fn service<R: Repository>(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/v1").configure(users::service::<R>));
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

pub fn config<R: Repository>(cfg: &mut web::ServiceConfig) {
    // este es el get anterior
    cfg.service(service2);

    // este es el get como en v1 pasando todo
    cfg.service(web::scope("/user2").route("{user_id}", web::get().to(users::get::<R>)));
}
