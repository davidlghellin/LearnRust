use dotenv::dotenv;
use std::env;

pub struct Config {
    pub puerto: u16,
    pub host: String,
}
pub fn read_config() -> Config {
    dotenv().ok();
    Config {
        puerto: env::var("PORT").expect("Puerto no definido").parse().expect("No puedo convertir, puerto mal definido"),
        host: env::var("HOST").expect("Puerto no definido"),
    }
}
