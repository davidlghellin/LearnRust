#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello_param));

    // region --start server
    let addres = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">> Escuchadno en {addres}\n");
    axum::Server::bind(&addres)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion --start server
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");

    Html("<strong>hello world</strong>")
}

async fn handler_hello_param(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("<strong>hello {name}</strong>"))
}
