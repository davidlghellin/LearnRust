use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};

use crate::{Error, Result, web};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, playload: Json<LoginPlayLoad>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDLER");

    // TODO implementar logica real de autenticación de bd
    if (playload.username != "user" || playload.pwd != "pass") {
        return Err(Error::LoginFail);
    }

    // TODO implementar real token
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Crear el body
    let body: Json<Value> = Json(json!({
        "result":{
            "success":true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPlayLoad {
    username: String,
    pwd: String,
}
