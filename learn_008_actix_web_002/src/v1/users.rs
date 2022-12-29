use actix_web::{
    error::PathError,
    web::{self, PathConfig},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

use crate::repository::{Repository};
// .service(web::resource("/user/{user_id}").route(web::get().to(get_user)))

const PATH: &str = "/user";

pub fn service<R: Repository>(cfg: &mut web::ServiceConfig) {
    //cfg.service(version);
    cfg.service(
        web::scope(PATH)
            //Esta configuración la pasariamos a la que especifiquemos
            // .service(web::resource("/{user_id}")).app_data(PathConfig::default().error_handler(|err, _req| {
            //   actix_web::error::ErrorBadRequest(err)
            // })).route("{user_id}", web::get().to(get_user))
            //Esta configuración la pasamos a todas las rutas
            .app_data(PathConfig::default().error_handler(
                // como hemos quitado el validador de la funcion de get_user, ahora el error del parseo del uuid lo vamos a devolver nosostros
                // devolveremos el error
                path_config_handler,
            ))
            // GET
            // PUT
            // POST
            // ....
            .route("{user_id}", web::get().to(get::<R>)),
    );
}

pub async fn get<R: Repository>(user_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.get_user(&user_id) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

fn path_config_handler(err: PathError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::ErrorBadRequest(err)
}
