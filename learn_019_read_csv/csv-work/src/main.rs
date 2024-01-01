use csv;
use log::*;
use serde::Deserialize;
use std::error::Error;

use chrono::Local;
use std::fs::File;
use std::io::Write;

use std::time::Instant;

/*
https://crates.io/crates/csv
https://docs.rs/csv/latest/csv/
https://docs.rs/csv/1.3.0/csv/cookbook/index.html
*/

#[derive(Debug, Deserialize)]
struct Record {
    nombre: String,
    apellido: String,
    id: Option<u64>,
}

fn read_name(path: &str) -> Result<(), Box<dyn Error>> {
    debug!("Nombre del fichero: {}", path);
    let mut reader = csv::Reader::from_path(path)?;
    // let mut reader = csv::Reader::from_reader(io::stdin()); // cargo run < ../nombres.csv
    for result in reader.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        debug!("{:?}", record);
        println!(
            "{:?}\t{:?} con id: {:?}",
            record.nombre,
            record.apellido,
            record.id.unwrap()
        );
    }
    Ok(())
}

fn main() {
    // Para guardar en fichero
    let target = Box::new(File::create("../log/log.log").expect("Can't create file"));

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    // FIN Para guardar en fichero

    // std::env::set_var("RUST_LOG", "debug");
    // env_logger::init();
    info!("Ini programa");

    let now = Instant::now();
    // if let Err(e) = read_from_file("../customers.csv") {
    if let Err(e) = read_name("../input/nombres.csv") {
        eprintln!("{}", e); // https://doc.rust-lang.org/std/macro.eprintln.html
    }
    let elapsed = now.elapsed();
    info!("Fin programa: {:.2?}", elapsed);
}
