use actix_web::{get, web, Responder};
use serde::Deserialize;

use super::super::domain::Playlist;

#[get("/playlist")]
async fn playlist() -> impl Responder {
    let mut playlist: Vec<Playlist> = vec![];

    let p1: Playlist = Playlist {
        name: "Diciembre 2022".to_string(),
        songs: vec![],
    };
    playlist.push(p1);

    //HttpResponse::Ok(web::Json(playlist) // revisar como devolver en https://actix.rs/docs/response/#json-response
    web::Json(playlist)
}

// extractores
// https://actix.rs/docs/extractors/#path
#[derive(Deserialize)]
struct Info {
    id: usize,
}

#[get("/playlist/{id}")]
async fn get_playlist(info: web::Path<Info>) -> impl Responder {
    let playl: Vec<Playlist> = vec![
        Playlist {
            name: "Enero 2023".to_string(),
            songs: vec![],
        },
        Playlist {
            name: "Febrero 2023".to_string(),
            songs: vec![],
        },
    ];

    let p1: Playlist = playl[info.id].clone();

    web::Json(p1)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playlist);
    cfg.service(get_playlist);
}
