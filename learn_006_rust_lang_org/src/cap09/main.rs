use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //a();
    let f: Result<File, std::io::Error> = File::open("src/cap09/hola.txt");
    let f2 = match f {
        Ok(file) => file,
        Err(error) => {
            //panic!("problema abriendo el fichero: {:?}",error)
            match error.kind() {
                ErrorKind::NotFound => match File::create("hola.txt") {
                    Ok(fc) => fc,
                    Err(error) => panic!("problema abriendo el fichero: {:?}", error),
                },
                _otro => panic!(""),
            }
        }
    };
    let metadata: Result<std::fs::Metadata, std::io::Error> = f2.metadata();
    println!("{:?}", metadata);
}
fn a() {
    b();
}
fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("programa roto por pasar 22") // RUST_BACKTRACE=1 cargo run --bin cap09
    }
}
