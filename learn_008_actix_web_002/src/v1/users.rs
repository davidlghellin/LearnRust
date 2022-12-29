use actix_web::{
    error::PathError,
    web::{self, PathConfig},
    HttpRequest, HttpResponse,
};
use uuid::Uuid;

use crate::repository::Repository;
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

#[cfg(test)]
use mockall::{automock, mock, predicate::*};
#[cfg_attr(test, automock)]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

#[cfg(test)]
mod tests {
    use crate::user::User;

    use super::*;
    // use mockall::predicate::*;
    // use mockall::*;

    #[test]
    fn mytest() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);
        assert_eq!(5, mock.foo(4));
    }

    mock! {
        CustomRepo{}
        impl Repository for CustomRepo{
            fn get_user(&self, user_id:&uuid::Uuid) -> Result<User,String>;
        }
    }

    #[actix_rt::test]
    async fn it_works() {
        let user_id = uuid::Uuid::new_v4();
        let user_name = "Mi nombre";

        let mut repo = MockCustomRepo::default();
        repo.expect_get_user().returning(move |id| {
            let mut user = User::new(user_name.to_string(), (1977, 3, 10));
            user.id = *id;
            Ok(user)
        });

        let /*mut*/ result = get(web::Path::from(user_id), web::Data::new(repo)).await;
        println!("{:?}", result);
        // TODO no he conseguido el mock, puede que sea por versiones
    }
}
