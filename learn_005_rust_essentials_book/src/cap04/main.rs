use std::io;

fn main() {
    // input();
    patter_match();
}

#[warn(dead_code)]
fn input() {
    println!("¿Como te llamas? noble guerrero");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        //.ok()
        .expect("Fallamos al leer la línea");
    println!("{}, poderoso nombre!", buf.trim());

    println!("Edad??");
    let mut buf2 = String::new();
    io::stdin()
        .read_line(&mut buf2)
        .expect("Fallo al leer el num");

    let input_num2: Result<u32, _> = buf2.trim().parse();

    println!("{}, buena edad!", input_num2.unwrap());

    println!("------");
    println!("Edad2??");
    let mut buf3 = String::new();
    io::stdin()
        .read_line(&mut buf3)
        .expect("Fallo al leer el num");

    let input_num3: Result<u32, _> = buf3.trim().parse();

    // ------------------------
    // Matching patterns
    // ------------------------
    match input_num3 {
        Ok(num) => println!("{} OK", num),
        Err(ex) => println!("Por favor introduzca un número entero! {}", ex),
    };

    println!("------");
    let input_num4: Result<u32, _> = buf3.trim().parse();
    if let Ok(val) = input_num4 {
        println!("Matched {:?}!", val);
    } else {
        println!("No match!");
    }

    println!("------");
    let input_num5: Result<u32, _> = buf3.trim().parse();
    let num = input_num5.unwrap_or(0);
    // Al poner el código siguiente "clippy" nos sugería el de la línea anterior
    //match input_num5 {
    //    Ok(num) => num,
    //    Err(_) => 0,
    //};

    println!("Numero {:?}!", num);

    println!("------");
    println!("Bucle infinito sino es 42 (siempre que sea num)");
    let mut buf6 = String::new();
    io::stdin()
        .read_line(&mut buf6)
        .expect("Fallo al leer el num");
    let input_num6: Result<u32, _> = buf6.trim().parse();

    // Bucle infinito si no es 42
    while let Ok(val) = input_num6 {
        println!("Matched {:?}!", val);
        if val == 42 {
            break;
        }
    }
}

fn patter_match() {
    let magician = "Gandalf";
    match magician {
        "Gandalf" => println!("A good magician!"),
        "Sauron" => println!("A magician turned bad!"),
        _ => println!("No magician turned up!"), // si no ponemos esta opción nos dirá q no es exaustiva y nos dará error
    }

    let magical_number: i32 = 42;
    match magical_number {
        // Match a single value
        1 => println!("Unity!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("Ok, these are primes"),
        // Match an inclusive range
        // 40..=42 => println!("It is contained in this range"),
        num @ 40..=42 => println!("{} is contained in this range", num),
        // Handle the rest of cases
        _ => println!("No magic at all!"),
    }

    let loki: (&str, bool, u32) = ("Loki", true, 800u32);
    match loki {
        (name, demi, _) if demi => {
            print!("This is a demigod ");
            println!("called {}", name);
        }
        (name, _, _) if name == "Thor" => println!("This is Thor!"),
        (_, _, pow) if pow <= 1000 => println!("This is a powerless god"),
        _ => println!("This is something else"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn eje() {
        assert_eq!(2, 2)
    }
}
