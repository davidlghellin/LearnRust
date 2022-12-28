use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::repository::RepositoryInjector;
// .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))

const PATH: &str = "/user";

pub fn service(cfg: &mut web::ServiceConfig) {
    //cfg.service(version);
    cfg.service(
        web::scope(PATH)
            // GET
            // PUT
            // POST
            // ....
            .route("{user_id}", web::get().to(get_user)),
    );
}

pub async fn get_user(
    user_id: web::Path<String>,
    repo: web::Data<RepositoryInjector>,
) -> HttpResponse {
    if let Ok(parsed_user_id) = Uuid::parse_str(&user_id) {
        match repo.get_user(&parsed_user_id) {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().body("Not found"),
        }
    } else {
        HttpResponse::BadRequest().body("Invalid UUID")
    }
}
