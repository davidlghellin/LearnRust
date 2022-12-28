use actix_web::{get, web, HttpResponse, Responder};

// Para configurar los endpoints básicos
#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("x-hello", "world"))
        .body("1.0.0\n")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(version);
}

// https://actix.rs/docs/testing#integration-tests
#[cfg(test)]
mod tests {
    use actix_web::{dev::ServiceResponse, http::StatusCode};
    use actix_web::{test, web, App};

    use super::*;

    #[actix_rt::test]
    async fn g_a_deck() {
        use actix_web::{test, App};

        // Con esto tenemos nuestro servicio
        let app = App::new().configure(config);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/version").to_request();
        // hacemos la llamada
        let resp: ServiceResponse = test::call_service(&app, req).await;

        // Comprobar cstatus code
        assert_eq!(resp.status(), 200);
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.status().is_success());

        // let data = resp.headers().get("x-hello").map(|h| h.to_str().ok()).flatten();
        // Comprobamos los headers
        let data: Option<&str> = resp.headers().get("x-hello").and_then(|h| h.to_str().ok());
        assert_eq!(data, Some("world"));
    }

    #[actix_web::test]
    async fn test_index_post() { 
        let app =
            test::init_service(App::new().route("/falla", web::get().to(|| async { "" }))).await;
        let req = test::TestRequest::post().uri("/falla").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
