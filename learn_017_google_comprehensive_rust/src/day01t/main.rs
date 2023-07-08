const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");

    /* Heap Stack */
    // Crear un String pone datos de tamaño fijo en la pila y de tamaño dinámico datos sobre el montón:
    let s1 = String::from("Hello");
    /*
    Stack                 Heap
    s1:                       ___________
        ptr ------------->    |h|e|l|l|o|
        len      = 5          ___________
        capacity = 5
     */
    println!("{}", s1);

    /* move-semantics */
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    //La asignación de s1 a s2 transfiere la propiedad.
    println!("s2: {s2}");
    // println!("s1: {s1}"); // Los datos fueron movido desde s1 y s1 ya no es accesible.
    // Cuando s1 sale del alcance, no pasa nada: no tiene propiedad.
    // Cuando s2 sale del alcance, los datos de cadena se liberan.
    // Siempre hay exactamente un enlace variable que posee un valor.
    // En Rust, los clones son explícitos ( mediante el uso clone).
    let s3: String = s2.clone();
    println!("s2: {s2}");
    println!("s3: {s3}");

    /* move en funciones */
    fn say_hello(name: String) {
        println!("Hello {name}")
    }
    let name = String::from("Alice");
    say_hello(name); // pasamos name pero entonces al salir de la función hemos perdido la variable
                     // say_hello(name);
                     // ahora pasamos la referencia de la variable
    fn say_hello2(name: &String) {
        println!("Hello {name}")
    }
    let name2 = String::from("Bob");
    say_hello2(&name2);
    say_hello2(&name2);
    // también podriamos enviar un name.clone()

    // Copy copia de forma predeterminada, si quisieramos el clone explicito
    #[derive(Copy, Clone, Debug)]
    struct Point(i32, i32);

    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");

    println!("p2: {p2:?}");
    // Copiar y clonar no son lo mismo
    println!();

    // Borrowing - prestamo
    #[derive(Debug)]
    struct Point2(i32, i32);

    fn add(p1: &Point2, p2: &Point2) -> Point2 {
        Point2(p1.0 + p2.0, p1.1 + p2.1)
    }
    let p1 = Point2(3, 4);
    let p2 = Point2(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
}
