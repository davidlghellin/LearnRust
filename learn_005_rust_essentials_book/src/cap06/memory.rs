use std::fmt::{Display, Error};

// Puede que sea el cap más importante
fn main() {
    /*
    Cuando se incia un programa, por defecto se concede un trozo de memoria de 2MB a la pila(stack).
    El programa almacenará las variables locales y los parámetros de las funciones.

    Los tipos de tamaño dinámico (cadenas, arrays) no pueden almacenarse en la pila,
    para esos valores se almacena en el montón(heap).

    Es es posible, el uso de la pila es preferible, el acceso es más eficiente.

    - Tiempo de vida de las variables
    -- Es el scope donde están definidas {}
    -
    */
    /*
    struct MagicNumber {
    value: u64
    }
    impl Copy for MagicNumber {}
    */
    #[derive(Clone, Copy)]
    struct MagicNumber {
        value: u64,
    }
    let mag = MagicNumber { value: 42 };
    let mag2 = mag;
    println!("{:?}", &mag as *const MagicNumber); // address is 0x23fa88
    println!("{:?}", &mag2 as *const MagicNumber); // address is 0x23fa80

    let mag3 = mag.clone();
    println!("{:?}", &mag3 as *const MagicNumber); // address is 0x23fa78
    println!();

    // ---------
    // Punteros
    // ---------
    // m es un puntero q tiene la dir de n
    let n = 42i32;
    let m = &n; // obtenemos la direccion de n
    println!("The address of n is {:p}", m);
    println!("The value of n is {}", *m);
    println!("The value of n is {}", m);
    /*
    ¿por qué necesitamos punteros? Cuando trabajamos con valores asignados dinámicamente,
    como una cadena, que puede cambiar de tamaño, la dirección de memoria de ese valor no es
    en tiempo de compilación. Debido a esto, la dirección de memoria necesita ser calculada en
    tiempo de ejecución. Por lo tanto, para poder realizar un seguimiento de la misma, necesitamos un puntero cuyo valor se
    cambiará cuando cambie la ubicación de String en memoria.
    */
    let q = &42;
    println!("{}", square(q)); // 1764
    fn square(k: &i32) -> i32 {
        *k * *k
    }

    // Referencias y Prestamos
    let cadena: String = String::from("Hola");
    fn get_longitud(referencia_cadena: &String) -> usize {
        referencia_cadena.len()
    }
    println!("Texto: {} || longitud: {}", cadena, get_longitud(&cadena));
    // En el cap05 por ejemplo estabamos creando muchas variables porque cuando haciamos la llamada a la función al terminar, la variable se destruía
    // Con los tipos primitivos eso no pasa, porq es barato y se copian
    // Veamos un ejemplo con un struct propio
    #[derive(Debug)]
    struct Ejemp {
        x: String,
    }
    let ejemp01 = Ejemp {
        x: String::from("Cadena"),
    };
    println!("{:?}", ejemp01);
    fn mas_ejem(valor: &Ejemp, texto_add: String) -> String {
        format!("{} {}", valor.x, texto_add)
    }
    // Como hemos definido en la función anterior que Ejemp lo pasamos como referencia, podemos luego volver a usar esa misma variable
    println!("{:?}", mas_ejem(&ejemp01, String::from("mas")));
    println!("{:?}", ejemp01);
    //Pag95
}
