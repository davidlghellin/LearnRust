use std::fmt::Debug;

#[derive(Debug)]
struct User {
    nombre_usuario: String,
    email: String,
    count_iniciado_sesion: u64,
    activo: bool,
}
fn main() {
    let mut user1: User = User {
        nombre_usuario: String::from("value"),
        email: String::from("value"),
        activo: true,
        count_iniciado_sesion: 1,
    };
    let nombre1: String = user1.nombre_usuario;
    user1.nombre_usuario = String::from("cambio");
    println!("User1: {:?}", user1);

    let user2: User = build_user(String::from("nombre"), String::from("correo"));
    println!("User2: {:?}", user2);

    let user3: User = User {
        nombre_usuario: String::from("otro"),
        email: String::from("otro"),
        ..user2
    };
    println!("User3: {:#?}", user3);

    // Structuras de tupla
    struct ColorRGB(i32, i32, i32);
    struct Point(i32, i32, i32);

    let alto1 = 30;
    let ancho1 = 50;

    println!(
        "El area del rectangulo es {} cuadrados pixeles.",
        area(alto1, ancho1)
    );

    let rect = Rectangulo {
        alto: 30,
        ancho: 50,
    };
    println!("{:#?}", rect);

    println!(
        "El area del rectangulo es {} cuadrados pixeles.",
        area_rectangulo(&rect)
    );

    println!(
        "El area del rectangulo es {} cuadrados pixeles.",
        rect.area()
    );

    let rect1: Rectangulo = Rectangulo {
        alto: 20,
        ancho: 40,
    };
    let rect2: Rectangulo = Rectangulo {
        alto: 60,
        ancho: 60,
    };
    println!("Cabe dentro el rect1: {}", rect.encaja_dentro(&rect1));
    println!("Cabe dentro el rect2: {}", rect.encaja_dentro(&rect2));

    let rect3: Rectangulo = Rectangulo::cuadrado(30);
    println!("{:#?}", rect3);
}

fn build_user(nombre: String, correo: String) -> User {
    User {
        nombre_usuario: nombre,
        email: correo,
        activo: true,
        count_iniciado_sesion: 1,
    }
}

fn area(alto: u32, ancho: u32) -> u32 {
    alto * ancho
}

#[derive(Debug)]
struct Rectangulo {
    alto: u32,
    ancho: u32,
}
impl Rectangulo {
    fn area(&self) -> u32 {
        self.alto * self.ancho
    }
    fn encaja_dentro(&self, otro: &Rectangulo) -> bool {
        self.alto > otro.alto && self.ancho > otro.ancho
    }
}

impl Rectangulo {
    fn cuadrado(n: u32) -> Rectangulo {
        Rectangulo { alto: n, ancho: n }
    }
}

// En la función estamos prestando la variable para que la función haga lo que tenga que hacer
fn area_rectangulo(rectangulo: &Rectangulo) -> u32 {
    rectangulo.alto * rectangulo.ancho
}
