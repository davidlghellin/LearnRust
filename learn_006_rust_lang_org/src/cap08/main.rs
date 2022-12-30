use colored::*;
use log::{error, info, warn};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Activamos logs");

    let a: [u32; 3] = [1, 2, 3];

    info!("{:?}", a);

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    info!("{:?}", v);

    let v2 = vec![1, 2, 3, 4, 5];
    let tercero = &v2[2];

    warn!("El valor del tercer indice es: {:?}", tercero);

    // let nos_hemos_salid = &v2[20]; // detecta el fallo en tiempo de ejecución
    // para acceder sin miedo usariamos esta forma de obtener el valor
    match v2.get(20) {
        Some(value) => warn!("El valor del tercer indice es: {:?}", value),
        None => error!("{}", "Nos hemos salido".red()),
    }
    // cuadno accedemos a elemtos de un vector, obtenemos una referencia a ese elemento

    // Iteramos para imprimir
    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v {
        // estamos cambiando el valor de los valores del vector
        *i *= 10; // como tenemos la referenca en v, tenemos que desreferenciar, por ellos lo del *i
        println!("i*10: {}", i);
    }
    // aquí podemos ver los valores ya cambiados
    for i in &v {
        println!("{}", i);
    }

    //////////////
    #[derive(Debug)]
    enum HojaCalculo {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // como tenemos un enum con distintos valores podemos hacer un vector con distintos valores
    let fila = vec![
        HojaCalculo::Int(3),
        HojaCalculo::Float(3.3),
        HojaCalculo::Text(String::from("value")),
    ];
    info!("fila: {:?}", fila);
    match &fila[1] {
        HojaCalculo::Int(i) => info!("fila: {:?}", i),
        _ => warn!("No es un entero"),
    }

    // Las cadenas son complicadas de tratar con la conplejidad de ellas.
    // Las cadenas son almacenadas como colecciones de bytes codificados en UTF-8
    let s1: String = String::new();
    let s2: &str = "Contenido inicial";
    let s3: String = s2.to_string();
    let s4: String = String::from("Contenido incial");

    let s11 = String::from("hola, ");
    let s21 = String::from("mundo!");
    let s31: String = s11 + &s21;
    // info!("{:?}", s11); // no podemos tomarlo prestado porque lo hemos movido
    info!("{:?}", s21);
    let s41: String = format!("{}{}", s21, s31);
    info!("{:?}", s41);

    // elemteos de un String
    let hola = String::from("hola");
    //let c:char= hola[0]; // en otros lenguajes sería algo así
    // recordemos que los utf-8 puden tener de 1 a 4 bytes
    // Rust no sabe que queremos recibir : Bytes, scalar values, grapheme clusters.
    // Bytes
    for i in hola.bytes() {
        println!("{}", i)
    }
    for i in hola.chars() {
        println!("{}", i)
    }
    println!();
    // unicode-segmentation, es el que queremos
    for i in hola.graphemes(true) {
        println!("{}", i)
    }

    // HasMap
    let azul: String = String::from("azul");
    let verde: String = String::from("verde");

    let mut puntuaciones: HashMap<String, i8> = HashMap::new();
    puntuaciones.insert(azul, 10);
    puntuaciones.insert(verde, 50);

    let nombre_azul: String = String::from("azul");
    let puntacion_azul: Option<&i8> = puntuaciones.get(&nombre_azul);
    info!("{:?}", puntacion_azul);

    for (key, value) in &puntuaciones {
        info!("{}: {}", key, value);
    }
    // sobreescribimos
    puntuaciones.insert(String::from("verde"), 0);

    // insertamos
    puntuaciones.entry(String::from("amarillo")).or_insert(0);
    // como existe no modifica el valor
    puntuaciones.entry(String::from("amarillo")).or_insert(100);

    for (key, value) in &puntuaciones {
        info!("{}: {}", key, value);
    }
    let cadena = "hola mundo estoy aqui mundo";
    let mut cuenta_palabras: HashMap<String, i8> = HashMap::new();

    for palabra in cadena.split_whitespace() {
        let n = cuenta_palabras.entry(palabra.to_string()).or_insert(0);
        // or_insert nos devuelve una referencia mutable, por eso se puede cambiar
        *n += 1;
    }
    info!("{:?}", cuenta_palabras);
}
