use actix_web::{web, App, HttpServer};

use actix_web::middleware::Logger;
use learn_007_actix_web_001::modules::music::domain::Playlist;
use log::info;

use learn_007_actix_web_001::config::{read_config, Config};
use learn_007_actix_web_001::infra;
use learn_007_actix_web_001::modules::music;
use learn_007_actix_web_001::state::MyState;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Activamos logs");

    // Añadimos la lectura de la configuración
    let configuracion: Config = read_config();
    let stack: Vec<Playlist> = vec![];
    let playlist = Arc::new(Mutex::new(stack));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(MyState {
                playlist: playlist.clone(),
            }))
            // añadimos el log
            .wrap(Logger::default())
            // al endpoin /hola le decimos que vamos a aceptar peticiones get y vamos a devolver esa cadena
            //.route("/hola", web::get().to(|| async { "Hola mundo\n" }))
            .service(
                // hacemos que todo esté dentro de /api
                web::scope("/api")
                    // hemos añadido la configuración de la infra
                    .configure(infra::endpoints::config)
                    .configure(music::infra::endpoints::config),
            )
    })
    .bind((configuracion.host, configuracion.puerto))?
    .run()
    .await
}
