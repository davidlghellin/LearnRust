use actix_web::{web, App, HttpServer};

use actix_web::middleware::Logger;
use log::info;

use learn_007_actix_web_001::config::{read_config, Config};
use learn_007_actix_web_001::infra;
use learn_007_actix_web_001::modules::music;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Activamos logs");

    // Añadimos la lectura de la configuración
    let configuracion: Config = read_config();

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
                    .configure(infra::endpoints::config)
                    .configure(music::infra::endpoints::config)
            )
    })
    .bind((configuracion.host, configuracion.puerto))?
    .run()
    .await
}
