#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello());

    // region --start server
    let addres = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">> Escuchadno en {addres}\n");
    axum::Server::bind(&addres)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion --start server
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello_param))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// `/hello`
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");

    Html("<strong>hello world</strong>")
}

// `/hello?name=david`
async fn handler_hello_param(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("world");
    Html(format!("<strong>hello {name}</strong>"))
}

// `/hello2/David`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("<strong>hello {name}</strong>"))
}