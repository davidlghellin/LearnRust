use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Telefono {
    numero: String,
}

#[derive(Serialize, Deserialize)]
struct Persona {
    nombre: String,
    edad: u8,
    numeros: Vec<Telefono>,
}

fn main() {
    let persona: Persona = Persona {
        nombre: String::from("Dav1d"),
        edad: 38,
        numeros: vec![
            Telefono {
                numero: String::from("+34 6565656565"),
            },
            Telefono {
                numero: String::from("+34 1221212121"),
            },
        ],
    };
    let json: String = serde_json::to_string(&persona).unwrap();
    println!("el json es: {}", json)
}
