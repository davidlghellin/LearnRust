use crate::modules::music::domain::Playlist;
use std::sync::{Arc, Mutex};

pub struct MyState {
    // Como tenemos varios cores, vamos a compartir la estructura en un par de tipos 
    pub playlist: Arc<Mutex<Vec<Playlist>>>,
}
