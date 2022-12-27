use actix_web::{web, App, HttpServer};

use actix_web::middleware::Logger;
use log::info;

use learn_007_actix_web_001::infra::endpoints::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Activamos logs");

    HttpServer::new(|| {
        App::new()
            // añadimos el log
            .wrap(Logger::default())
            // al endpoin /hola le decimos que vamos a aceptar peticiones get y vamos a devolver esa cadena
            //.route("/hola", web::get().to(|| async { "Hola mundo\n" }))
            .service(
                // hacemos que todo esté dentro de /api
                web::scope("/api")
                    // hemos añadido la configuración de la infra
                    .configure(config),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
