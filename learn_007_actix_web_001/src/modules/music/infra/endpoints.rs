use std::sync::MutexGuard;

use crate::modules::music::domain::Song;

use super::dtos::{CreatePlaylist, Info};
use actix_web::{get, post, web, Responder};

use super::super::domain::Playlist;
use crate::state::MyState;

#[get("/playlist")]
async fn playlist(data: web::Data<MyState>) -> impl Responder {
    // Vamos a tomar la variable y la bloqueamos
    let playlist: MutexGuard<Vec<Playlist>> = data.playlist.lock().expect("bad state");

    //HttpResponse::Ok(web::Json(playlist) // revisar como devolver en https://actix.rs/docs/response/#json-response
    web::Json(playlist.clone())
}

// extractores
// https://actix.rs/docs/extractors/#path

#[get("/playlist/{id}")]
async fn get_playlist(info: web::Path<Info>, data: web::Data<MyState>) -> impl Responder {
    let playl: MutexGuard<Vec<Playlist>> = data.playlist.lock().expect("bad state");

    let p1: Playlist = playl[info.id].clone();

    web::Json(p1)
}

#[post("/playlist")]
async fn create_playlist(
    dto: web::Json<CreatePlaylist>,
    data: web::Data<MyState>,
) -> impl Responder {
    let mut play: MutexGuard<Vec<Playlist>> = data.playlist.lock().expect("bad state");

    let p1 = Playlist {
        name: dto.name.clone(),
        songs: vec![Song {
            name: dto.song.clone(),
            autor: String::from("Desconocido"),
            duration_ms: 0,
        }],
    };
    play.push(p1.clone());

    web::Json(p1)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playlist);
    cfg.service(get_playlist);
    cfg.service(create_playlist);
}
