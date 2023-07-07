fn main() {
    let vect: Vec<i32> = vec![1, 2, 3, 4];
    let sum: i32 = vect.iter().fold(0, |sum: i32, x: &i32| sum + x);
    println!("La suma es {}", sum);

    let mut x: i32 = 10; //representa una referencia mutable a la que se puede vincular valores diferentes
    let ref_x: &mut i32 = &mut x; //representa una referencia a un valor mutable
    println!("x: {ref_x}");
    *ref_x = 20;
    println!("x: {x}");

    // Slices
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
    println!("s: {:?}", &a[2..4]);

    // imprimimos todos los elementos
    println!("s: {:?}", &a[0..a.len()]);
    println!("s: {:?}", &a[..a.len()]);

    // imprimimos desde el 2 al final
    println!("s: {:?}", &a[2..]);
    println!("s: {:?}", &a[2..a.len()]);

    /* String */
    // &str   -> Una referencia inmutable a una rodaja de cuerda.
    // String -> Un búfer de cadena mutable.

    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");

    // Métodos

    let mut rect = Rectangle {
        width: 10,
        height: 5,
    };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());

    println!("new rectangle mut: {}", Rectangle::new(10, 2).area());
    println!("new rectangle inm: {}", rect.inc_width_inmut(2).area());
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        // ese &mut self es para modificar el propio objeto, mmmm no me gusta por ser mutable
        self.width += delta;
    }
    fn inc_width_inmut(&self, delta: u32) -> Rectangle {
        Rectangle::new(self.width + delta, self.height)
    }
}
