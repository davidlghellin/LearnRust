use serde::{Deserialize, Serialize};

/*
https://crates.io/crates/serde_json/1.0.1
https://docs.rs/serde_json/1.0.1/serde_json/
*/

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
    let json = r#"
{
    "nombre": "David",
    "edad": 38,
    "numeros": [
        {
            "numero": "+34 21212121212121"
        },
        {
            "numero": "+34 54545454545454"
        },
        {
            "numero": "+34 87878787878787"
        }
    ]
}
"#;
    let parsed: Persona = read_json_typed(json);

    println!("\n\n");
    println!(
        "El primer número de telefono es:  {}",
        parsed.numeros[0].numero
    );

    println!("-----");
    parsed.numeros.into_iter().for_each(|tel: Telefono| {
        println!("Un telefono asociado {}", tel.numero);
    });
    /*
    for tel in parsed.numeros {
        println!("Un telefono asociado {}", tel.numero);
    }
    */
}

fn read_json_typed(raw_json: &str) -> Persona {
    let parsed: Persona = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
