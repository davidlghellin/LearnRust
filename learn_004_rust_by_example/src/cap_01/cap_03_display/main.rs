use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    // stackoverflow example siendo el mismo debug y display
    let d1 = MyDate {
        year: 2008,
        month: 9,
        day: 10,
    };
    let d2 = MyDate {
        year: 2008,
        month: 9,
        day: 10,
    };

    println!("Date : {}", d1); // requires fmt::Display
    println!("Debug: {:?}", d1);

    assert_eq!(d1, d2); // requires fmt::Debug
}

struct MyDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl PartialEq for MyDate {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl fmt::Display for MyDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl fmt::Debug for MyDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
