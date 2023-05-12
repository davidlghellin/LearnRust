#![allow(unused)]

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("<strong>hello world</strong>") }),
    );
    // region --start server
    let addres = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">> Escuchadno en {addres}\n");
    axum::Server::bind(&addres)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion --start server
}
