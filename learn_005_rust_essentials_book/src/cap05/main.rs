use std::fmt;
use std::fmt::{Display, Error};
use std::io;
use std::ops::Range;

fn main() {
    hof();
    println!();
    iterators();
    println!();
    gnereic();
    println!();
    error_handling();
    println!();
    method_on_struct();
    println!();
    mis_traits();
    println!();
}
fn hof() {
    let mut strength = 26;
    println!("My tripled strength equals {}", triples(strength));
    println!("My strength is still {}", strength);
    strength = triples(strength);
    println!("My strength is now {}", strength);

    // supongamos que queremos hacer el triple del triple
    strength = again(triples, strength);
    println!("I got so lucky to turn my strength into {}", strength);
    // La función again, es de Higher-order function
    // ya que toma una función como parámetro
    // veamos lo mismo con lambdas/clojure
    strength = 78;
    let triples = |n| 3 * n;
    strength = again(triples, strength);
    println!("My strength is now {}", strength);

    let x: i32 = 42;
    let print_add = |s| {
        println!("x is {}", x);
        x + s
    };
    let res = print_add(strength);
    // here the closure is called and "x is 42" is printed
    println!("{:?}", res);
}

fn triples(s: i32) -> i32 {
    3 * s
}

fn again<F: Fn(i32) -> i32>(f: F, s: i32) -> i32 {
    f(f(s))
}

// ---------
// ITERATORS
// ---------
fn iterators() {
    let mut rng = 0..7;
    println!("> {:?}", rng.next()); // prints Some(0)
    println!("> {:?}", rng.next()); // prints Some(1)
    for n in rng {
        print!("{:?} - ", Some(n).unwrap());
    } //prints 2 - 3 - 4 - 5 - 6 -
    println!();
    println!();

    let aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
    for alien in aliens.iter() {
        print!("{} / ", alien) // process alien
    }
    println!();
    for alien in &aliens {
        print!("{} / ", alien)
    }
    println!();

    // ----------------------
    // Consumers and adapters
    // ----------------------
    // Los iteradores son perezosos y necesitan ser llamados desde un consumer

    let rng: Range<i32> = 0..1_000;
    let rngvec: Vec<i32> = rng.collect::<Vec<i32>>(); // collect recoge todos los elementos
                                                      // let rngvec: Vec<i32> = rng.collect();
    println!("{:?}", rngvec);
    println!();

    // definimos mutable la variable por el clousure
    let forty_two: Option<i32> = (0..1_000).find(|n: &i32| *n >= 42);
    println!("{:?}", forty_two); // prints out Some(42)
    println!();

    // queremos todos los numers pares
    let rng_even: Vec<i32> = (0..1_000)
        .filter(|n: &i32| is_even(*n))
        .collect::<Vec<i32>>();
    println!("{:?}", rng_even);
    println!();

    // Ahora queremos aplicar el cubo a los valores del filtro de si es par

    let rng_even_pow3: Vec<i32> = (0..1_000)
        .filter(|n: &i32| is_even(*n))
        .map(|n: i32| n * n * n)
        .take(5)
        .collect::<Vec<i32>>();
    println!("{:?}", rng_even_pow3);
    println!();

    // Fold, resultado que va acumulando en el valor inicial
    let sum: i32 = (0..101).fold(0, |sum, n| sum + (n + 1));
    println!("{}", sum);
    println!();
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn gnereic() {
    // -------------------------------------
    // Generic data structures and functions
    // -------------------------------------
    // Definimos Pair con dos elementos, pero siento genérico
    #[derive(Copy, Clone)]
    struct Pair<T> {
        first: T,
        second: T,
    }
    // usamos para enteros
    let magic_pair: Pair<u32> = Pair {
        first: 7,
        second: 42,
    };
    // usamos para string
    let pair_of_magicians: Pair<&str> = Pair {
        first: "Gandalf",
        second: "Sauron",
    };
    fn second<T>(pair: Pair<T>) -> T {
        pair.second
    }
    println!("{}", second(magic_pair));
    println!("{}", second(pair_of_magicians));
    println!();
    let a = second(magic_pair); // Para poder usar la variable dos veces hemos añadido #[derive(Copy, Clone)]
    println!("{}", a);
    println!();

    // Veamos Option como está definico, lo renombro
    enum MyOption<T> {
        Some(T),
        None,
    }

    #[derive(Debug)]
    struct Person {
        name: &'static str,
        id: i32,
    }
    let p1 = Person {
        name: "James Bond",
        id: 7,
    };
    let p2 = Person {
        name: "Vin Diesel",
        id: 12,
    };
    let p3 = Person {
        name: "Robin Hood",
        id: 42,
    };
    let op1: Option<Person> = Some(p1);
    let pvec: Vec<Option<Person>> = vec![Some(p2), Some(p3), None];

    println!("{:?}", op1);
    println!("{:?}", pvec);
    println!();

    // Ahora como está definido Result
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }
    println!();

    println!("Introduce un entero");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        //.ok()
        .expect("Fallamos al leer la línea");
    let input_num: Result<u32, _> = buf.trim().parse();
    println!("input_num: {:?}", input_num);
    match input_num {
        Ok(num) => println!("Del match {}", num),
        Err(ex) => println!("Please input an integer number! {}", ex),
    };
    println!();

    fn sqroot(r: f32) -> Result<f32, String> {
        if r < 0.0 {
            return Err("Number cannot be negative!".to_string());
        }
        Ok(f32::sqrt(r))
    }
    let m = sqroot(42.0);
    match m {
        Ok(sq) => println!("The square root of 42 is {}", sq),
        Err(str) => println!("{}", str),
    }
}

//Error handling
fn error_handling() {
    let x = 3;
    let y = 0;
    // x / y;       //attempt to divide `3_i32` by zero
    // Option<T>
    // Result<T, E>
}

fn method_on_struct() {
    struct Alien {
        health: u32,
        damage: u32,
    }
    impl Alien {
        // método estático para hacer la creación
        fn new(mut h: u32, d: u32) -> Alien {
            // constraints:
            if h > 100 {
                h = 100;
            }
            Alien {
                health: h,
                damage: d,
            }
        }
        // otro método estático
        fn warn() -> &'static str {
            "Leave this planet immediately or perish!"
        }
    }
    // hemos separado en este caso para separar los métodos estáticos.
    impl Alien {
        // función que tiene acceso al struct
        fn attack(&self) {
            println!(
                "I attack! Your health lowers with {} damage points.",
                self.damage
            );
        }
        // función que tiene acceso al valor de las propiedades que y las va a cambiar
        // necesita que la instancia sea mutable
        fn attack_and_suffer(&mut self, damage_from_other: u32) {
            self.health -= damage_from_other;
        }
    }

    let bork = Alien {
        health: 100,
        damage: 5,
    };
    // Como ver el print
    impl fmt::Display for Alien {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Salud: {}, daño: {}", self.health, self.damage)
        }
    }
    impl fmt::Debug for Alien {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Salud: {salud} del poderoso Bicho, daño: {danyo}",
                salud = self.health,
                danyo = self.damage
            )
        }
    }
    println!("El monstruo1 {}", bork);
    let berserk = Alien::new(150, 15);
    println!("El monstruo2 {:?}", berserk); // Debug
    println!("El aviso de los monstrous: {}", Alien::warn());
    berserk.attack();

    let mut mut_berserk = Alien::new(150, 15);
    println!("El monstruomutable {}", mut_berserk);
    mut_berserk.attack_and_suffer(31);
    println!("El monstruomutable {}", mut_berserk);
    println!();
}

fn mis_traits() {
    // un trait solo contiene la definición
    trait Monster {
        fn attack(&self);
    }
    struct Alien_2 {
        health: u32,
        damage: u32,
    }
    impl Monster for Alien_2 {
        fn attack(&self) {
            println!(
                "I attack! Your health lowers with {} damage points.",
                self.damage
            );
        }
    }
    #[derive(Debug)]
    struct Zombies {
        health: u32,
        damage: u32,
    }
    impl Monster for Zombies {
        fn attack(&self) {
            println!(
                "I bite you! Your health lowers with {} damage points.",
                2 * self.damage
            );
        }
    }
    trait Monster2 {
        fn new(hlt: u32, dam: u32) -> Self;
        fn noise(&self) -> &'static str;
        fn attacks_with_sound(&self) {
            println!(
                "The Monster attacks by making an awkward sound {}",
                self.noise()
            );
        }
    }
    impl Monster2 for Zombies {
        fn new(mut h: u32, d: u32) -> Zombies {
            // constraints:
            if h > 100 {
                h = 100;
            }
            Zombies {
                health: h,
                damage: d,
            }
        }

        fn noise(&self) -> &'static str {
            "Aaargh!"
        }
    }
    let zmb1 = Zombies {
        health: 75,
        damage: 15,
    };
    println!("Oh no, I hear: {}", zmb1.noise());
    zmb1.attack();
    println!("{:?}", zmb1); // hemos añadido la macro para que haga un print con los valores.

    // -----------------------
    // Using trait constraints
    // -----------------------
    // https://docs.rs/num-traits/latest/num_traits/
    fn sqroot<T: num_traits::Float>(r: T) -> Result<T, String> {
        if r < num_traits::zero() {
            return Err("Number cannot be negative!".to_string());
        }
        Ok(num_traits::Float::sqrt(r))
    }
    println!("The square root of {} is {:?}", 42.0f32, sqroot(42.0f32));
    println!("The square root of {} is {:?}", 42.0f64, sqroot(42.0f64));

    // daria error
    //println!("The square root of {} is {:?}", 42, sqroot(42) );
    // otra forma sería parametrizar con el where
    fn sqroot2<T>(r: T) -> Result<T, String>
    where
        T: num_traits::Float,
    {
        if r < num_traits::zero() {
            return Err("Number cannot be negative!".to_string());
        }
        Ok(num_traits::Float::sqrt(r))
    }
    println!();
    println!();
    // funcion que usa dos traits
    fn multc<T: Monster, U: Monster + Monster2>(x: T, y: U) {
        x.attack();
        y.attacks_with_sound();
    }
    let zmb2 = Zombies {
        health: 75,
        damage: 15,
    };
    multc(zmb1, zmb2);
    // otra forma
    fn multc2<T, U>(x: T, y: U)
    where
        T: Monster,
        U: Monster + Monster2,
    {
        x.attack();
        y.attacks_with_sound();
    }

    // Ejercicio :
    // Defnie el trait Draw, y define el entero y float
    trait Draw {
        fn draw(&self);
    }
    struct S1 {
        v: u32,
    }
    struct S2 {
        f: f32,
    }
    impl Draw for S1 {
        fn draw(&self) {
            println!("**{}**", self.v)
        }
    }
    impl Draw for S2 {
        fn draw(&self) {
            println!("++{}++", self.f)
        }
    }
    println!();
    println!();

    // Haz una función de los tipos que hayan implementado Draw
    fn dra_obj<T>(t: T)
    where
        T: Draw,
    {
        t.draw();
    }
    let s1 = S1 { v: 1 };
    dra_obj(s1);
    let s2 = S2 { f: 1.0 };
    dra_obj(s2);
}

fn trait_operator() {
    /*
    •Comparing objects (the Eq and PartialEq traits).
    •Ordering objects (the Ord and PartialOrd traits).
    •Creating an empty object (the Default trait).
    •Formatting a value using {:?} (the Debug trait, which defines a fmt method).
    •Copying an object (the Clone trait).
    •Adding objects (the Add trait, which defines an add method)
    */
}
