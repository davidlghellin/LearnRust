#![allow(unused)]

use crate::model::ModelController;

pub use self::error::{Error, Result};

use std::net::SocketAddr;

mod error;
mod model;
mod web;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    // Inicilizar el model controller
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region --start server
    let addres = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(">> Escuchadno en {addres}\n");
    axum::Server::bind(&addres)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion --start server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
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
