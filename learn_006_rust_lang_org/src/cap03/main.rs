fn main() {
    let x: i32 = 5;
    println!("El valor de la variable es {x}");

    // definimos una variable mutable
    let mut x_mut: i32 = 5;
    println!("El valor de la variable es {x_mut}");
    x_mut = 55;
    println!("El valor de la variable es {x_mut}");

    const CIEN_NUM: u32 = 100;
    println!("El valor de la constante es {CIEN_NUM}");

    // Tipos scalares
    // Integers
    // Números como flotante
    // Booleanos
    // Caracteres

    /*
    https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#integer-types
    Length	Signed	Unsigned
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize
    */
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("{decimal}");
    println!("{hex}");
    println!("{octal}");
    println!("{binary}");
    println!("{byte}");

    let cadena: &str = "cadena"; // El tamaño del string es fijo
    println!("{cadena}");
    let tupla: (&str, u8) = ("tupla", 22);
    let (v1_tupla, v2_tupla) = tupla;
    let v1_tupla_rep = tupla.0;
    let array: [i32; 3] = [1, 2, 34];
    let array2: [i32; 8] = [0; 8];
    println!("{}", v1_tupla);
    println!("{}", v2_tupla);
    println!("{}", v1_tupla_rep);
    println!("{:?}", array);
    println!("{:?}", array2);

    my_fun(2, 3);
    println!();
    my_loop(5);
    println!();
    my_while(5);
    println!();
    for_iter();
}

fn my_fun(x: i32, y: i32) -> i32 {
    println!("Otra funcion");
    println!("{}", x);
    println!("{}", y);
    x + y
}

fn my_loop(n: i8) {
    let mut counter = 0;
    loop {
        if counter == n {
            println!("hemos iterado: {}", counter + 1);
            break;
        }
        println!("el valor es:   {counter}");
        counter += 1;
    }
}
fn my_while(mut n: i8) {
    while n != 0 {
        println!("cuanta atrás:{}", n);
        n -= 1;
    }

    println!("GOOOO!!!!");
}
fn for_iter() {
    let v = [1, 2, 3, 4, 5];
    for element in v.iter() {
        println!("{}", element);
    }
    for element in 1..4 {
        println!("{}!", element);
    }
    
}
