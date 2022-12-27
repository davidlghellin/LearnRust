use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Song {
    pub name: String,
    pub autor: String,
    pub duration_ms: u16,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Song>,
}
