fn main() {
    println!("Hola, mundo!");

    let x: i32 = 3;
    let x2 = 3;
    println!("La variable es {}", x);
    println!("La variable es {}", x2);

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
}
