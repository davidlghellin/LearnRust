use csv;
use std::{error::Error, io};

use serde::Deserialize;
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

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn read_name(path: &str) -> Result<(), Box<dyn Error>> {
    // let mut reader = csv::Reader::from_path(path)?;
    let mut reader = csv::Reader::from_reader(io::stdin()); // cargo run < ../nombres.csv
    
    for result in reader.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    // if let Err(e) = read_from_file("../customers.csv") {
    if let Err(e) = read_name("../nombres.csv") {
        eprintln!("{}", e); // https://doc.rust-lang.org/std/macro.eprintln.html
    }
}
