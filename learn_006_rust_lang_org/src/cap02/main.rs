use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Juego de adivida el números");

    let num_secreto = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Introduzca su número para el juego");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Fallamos al leer la línea");
        //let numero: u32 = guess.trim().parse().expect("Por favor un número entero");
        let numero: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // no nos importa continua
        };

        println!("tu has introducido: {}", guess);

        match numero.cmp(&num_secreto) {
            Ordering::Less => println!("{}", "Muy bajo".red()),
            Ordering::Greater => println!("{}", "Demasiado grande".red()),
            Ordering::Equal => {
                println!("{}", "Has ganado!".green());
                break;
            }
        }
    }
}
