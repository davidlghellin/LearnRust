#![allow(unused_variables, dead_code)]

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    // println!("{x} * {y} = {}", multiply(x, y)); // falla por el casting no lo sabe hacer
    println!("{x} * {y} = {}", multiply(x.into(), y));
    println!("{x} * {y} = {}", multiply(i16::from(x), y));

    // array
    let array: [i32; 3] = [10, 20, 30];
    println!("array: {array:?}");

    print!("Iterating over array:");
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
    /* Ejercicio */
    /*
    Use lo anterior para escribir una función pretty_print que imprime bastante una matriz
    y una función transpose que transpondrá una matriz ( convertir filas en columnas ):
    */

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
    println!();
    pretty_print2(&transposed);
}
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut res: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            res[i][j] = matrix[j][i];
        }
    }
    res
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{row:?}");
    }
}

fn pretty_print2(matrix: &[[i32; 3]; 3]) {
    for i in 0..matrix.len(){
        print!("[");
        for j in 0..3{
            print!("{:?} ",matrix[i][j])
        }
        println!("]");
    }
}
