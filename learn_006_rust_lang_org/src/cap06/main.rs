enum DirIps {
    V4,
    V6,
}
fn route(ip_tipo: DirIps) {}
enum DirIps2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
struct IpDireccion {
    tipo: DirIps,
    ip_real: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//Podemos hacer funciones como en los struct en los enum
impl Message {
    fn una_fun() {
        println!("F")
    }
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}

fn main() {
    let ip_cuatro: DirIps = DirIps::V4;
    let ip_seis: DirIps = DirIps::V6;

    let localhost = IpDireccion {
        tipo: DirIps::V4,
        ip_real: String::from("127.0.0.1"),
    };

    let localhost2 = DirIps2::V4(127, 0, 0, 1);

    // ------
    // Option
    // ------
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("Cinco");

    let no_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // no se puede porq son distintos tipos
    let sum = x + y.unwrap_or(0);
    println!("La suma es: {sum}");

    value_in_cent(Coin::Quarter((Comunidades::Castilla)));

    // if let
    let some_3 = Some(3);
    if let Some(3) = some_3 {
        println!("Tres!");
    }
}

#[derive(Debug)]
enum Comunidades {
    Castilla,
    Madrid,
    RegMurcia,
    //...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Comunidades),
}

fn value_in_cent(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(Comunidades::Castilla) => {
            println!("Vamos Castilla");
            25
        }
        Coin::Quarter(com) => {
            println!("Comunidad de {:?}", com);
            25
        }
    }
}
fn suma_uno(x: Option<i32>) -> Option<i32> {
    match x {
        // autocompletado ctr+.
        Some(n) => Some(n + 1),
        _ => None,
    }
}
