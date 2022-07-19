---
title: Hello Rust: By Example
date: 2022-07-19
pin: false
tags:
- Rust
---

## Introduction

```rust
mod mod1;

fn main() {
    mod1::mod1fn();
}
// rustc main.rs
```

## Hello World

```rust
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
```

## Primitives

```rust
fn types() {
    let logical: bool = true;

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    // a_float = 1.1; // cannot be changed

    let mut inferred_type = 12; // i64
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;   // can be changed
    let mutable = true; // overwritten with shadowing
}

fn literals_and_operators() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    // 1 + 2 = 3
    // 1 - 2 = -1

    // println!("1 - 2 = {}", 1u32 - 2); // overflow

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    // true AND false is false
    // true OR false is true
    // NOT true is false

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    // 0011 AND 0101 is 0001
    // 0011 OR 0101 is 0111
    // 0011 XOR 0101 is 0110
    // 1 << 5 is 32
    // 0x80 >> 2 is 0x20

    println!("One million is written as {}", 1_000_000u32); // to improve readability
    // One million is written as 1000000
}

fn tuples() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64, 'a', true
    );
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);
    // long tuple first value: 1
    // long tuple second value: 2

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // tuple of tuples: ((1, 2, 2), (4, -1), -2)

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));
    // one element tuple: (5,)
    // just an integer: 5

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    // 1, "hello", 4.5, true
}

use std::mem;

fn arrays_and_slices() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("first element of the array: {}", xs[0]);
    println!("number of elements in array: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    // first element of the array: 1
    // number of elements in array: 5
    // array occupies 20 bytes

    let ys: [i32; 5] = [0; 5];
    for (idx, elem) in ys[1..4].iter().enumerate() {
        println!("{}: {}", idx, elem);
    }
    // 0: 0
    // 1: 0
    // 2: 0
}
```

## Custom Types

```rust
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
```

## Variable Bindings

## Types

## Conversion

## Expressions

## Flow of Control

## Functions

## Modules

## Crates

## Cargo

## Attributes

## Generics

## Scoping Rules

## Traits

## `macro_rules!`

## Error Handling

## Std Library Types

## Std mics

## Testing

## Unsafe Operations

## Compatibility

## Meta

