pub fn main() {
    structures();
    enums();
    web_events();
    type_aliases();
    linked_list();
    constants();
}

// Unit Struct, which useful for generics
struct Unit;

// Tuple Struct
struct Pair(i32, f32);

// Classic Struct
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn structures() {
    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // pair contains 1 and 0.1

    let Pair(integer, decimal) = pair;
    println!("destructured pair {:?} and {:?}", integer, decimal);
    // destructured pair 1 and 0.1

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    // point coordinates: (10.3, 0.4)

    let bottom_right_point = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right_point.x, bottom_right_point.y);
    // second point: (5.2, 0.4)

    let Point { x: left_edge, y: top_edge } = point;
    println!("destructured point: ({}, {})", left_edge, top_edge);
    // destructured point: (10.3, 0.4)

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right_point,
    };
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn enums() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    // zero is 0
    // one is 1

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    // roses are #ff0000
    // violets are #0000ff
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
    }
}

fn web_events() {
    use WebEvent::*;

    let pressed = KeyPress('x');
    let pasted  = Paste("my text".to_owned()); // `to_owned()` creates an owned `String` from a string slice.
    let click   = Click { x: 20, y: 80 };
    let load    = PageLoad;
    let unload  = PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    // pressed 'x'.
    // pasted "my text".
    // clicked at x=20, y=80.
    // page loaded
    // page unloaded
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn type_aliases() {
    let _x = Operations::Add;
}

use LinkedList::*;

enum LinkedList {
    Node(u32, Box<LinkedList>),
    Nil,
}

use std::fmt::format;

impl LinkedList {
    fn new() -> LinkedList {
        Nil
    }

    fn prepend(self, elem: u32) -> LinkedList {
        Node(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Node(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Node(elem, ref tail) => {
                format!("{}, {}", elem, tail.stringify())
            }
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn linked_list() {
    let mut list = LinkedList::new();
    println!("dummy linked list has length: {}", list.len());
    println!("{}", list.stringify());
    // dummy linked list has length: 0
    // Nil

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("final linked list has length: {}", list.len());
    println!("{}", list.stringify());
    // final linked list has length: 3
    // 3, 2, 1, Nil
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn constants() {
    // THRESHOLD = 5; // cannot modify a `const`

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    // This is Rust
    // The threshold is 10

    let n = 16;
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    // 16 is big
}
