mod in_memory;
mod domain;
mod swager;

use axum::{
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;
use log::LevelFilter;

use in_memory::rest_router;
use crate::domain::state::load_state;
use simple_logger::SimpleLogger;
use time::macros::format_description;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .with_timestamp_format(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"))
        .init().unwrap();

    log::warn!("This is an example message.");
    let app = Router::new()
        .merge(rest_router())
        // .layer(
        //     tower_http::cors::CorsLayer::new()
        //         .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        //         .allow_headers([CONTENT_TYPE])
        //         .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        // )
        .with_state(load_state());

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3030));
    log::info!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("stopped listening");
}
