fn main() {
    println!("Hola, mundo!");

    let x: i32 = 3;
    let x2 = 3;
    println!("La variable es {}", x);
    println!("La variable es {}", x2);
    println!();
    println!();
    println!();

    // variables mutables
    //let mut y_mut = 5;

    // constantes
    const PI: f32 = 31.1416;
    println!("PI es {}....", PI);

    // shadowing
    // Reasingacion de variable
    let var_redefinida = 3;
    println!("La variable es {}", var_redefinida);
    let var_redefinida = "4";
    println!("La variable es {}", var_redefinida);
    println!();
    println!();
    println!();

    tipos_datos();
    println!();
    println!();
    println!();

    funcion_param(5);
    println!();
    println!();
    println!();
    println!("{}", ocho());
}

fn tipos_datos() {
    println!("Tenemos 4 grandes tipos");
    println!("*  Enteros");
    println!("*  Coma flotante");
    println!("*  Booleanos");
    println!("*  Caracteres");

    let martini_emoji = '\u{1F378}';
    println!("El emoji es {}", martini_emoji);

    // tuplas y arrays
    let tupla: (i32, &str, i32, i32) = (1, "2", 3, 4);
    let (a, _, _, _) = tupla;
    println!("El primer valor de la tupla es: {}", a);

    let array: [i32; 3] = [1, 2, 3];
    println!("El segundo elemento es: {}", array[1])
}

fn funcion_param(i: i32) {
    println!("El valor es {}", i)
}

fn ocho() -> i8 {
    // return 8;
    8
}
