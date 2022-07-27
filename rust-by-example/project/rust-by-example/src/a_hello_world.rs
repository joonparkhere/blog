pub fn main() {
    intro();
    format_print();
    debug();
    display();
    formatting();
}

fn intro() {
    println!("Hello world!");
    println!("I'm a Rustacean!");
}

fn format_print() {
    eprintln!("Error print");

    print!("{} Month is ", 1);
    println!("{}", format!("{} days", 31));
    // 1 Month is 31 days

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Alice, this is Bob. Bob, this is Alice

    println!("{subject} {verb} {object}",
        subject="the quick brown fox",
        verb="jumps over",
        object="the lazy dog",
    );
    // the quick brown fox jumps over the lazy dog

    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);
    // Base 10 repr:               69420
    // Base 2 (binary) repr:       10000111100101100
    // Base 8 (octal) repr:        207454
    // Base 16 (hexadecimal) repr: 10f2c
    // Base 16 (hexadecimal) repr: 10F2C

    println!("{number:>5}", number=1);
    println!("{number:0>width$}", number=1, width=5);
    //     1
    // 00001

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:0>width$}");
    // 00001
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn debug() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);
    // Person {
    //     name: "Peter",
    //     age: 27,
    // }
}

use std::fmt::{self, Formatter, Display, write};

struct Point2D {
    x: f64,
    y: f64,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct MyList(Vec<i32>);

impl Display for MyList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

fn display() {
    let point = Point2D { x: 3.3, y: 2.2 };
    println!("Display: {}", point);
    // Display: x: 3.3, y: 2.2

    let v = MyList(vec![1, 2, 3]);
    println!("{}", v);
    // [0: 1, 1: 2, 2: 3]
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c
        )
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:0>2X}{1:0>2X}{2:0>2X}",
               self.red, self.green, self.blue
        )
    }
}

fn formatting() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    // Dublin: 53.348°N 6.260°W
    // Oslo: 59.950°N 10.750°E
    // Vancouver: 49.250°N 123.100°W

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
    // RGB (128, 255, 90) 0x80FF5A
    // RGB (0, 3, 254) 0x0003FE
    // RGB (0, 0, 0) 0x000000
}
