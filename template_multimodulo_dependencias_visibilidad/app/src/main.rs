use common::add_common;
use common::Second;
use common::Point;

fn main() {
    let suma = add_common(2,3);
    
    println!("La suma usando dependencia del módulo es {}",suma);
    println!("Dato para usar debug {:?}",Second::new(2));


    let point = Point { x: 1, y: 2 };
    println!("Dato para usar debug {:?}",point);
}
