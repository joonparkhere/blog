---
title: Hello Rust By Example
date: 2022-08-23
pin: false
tags:
- Rust
---

## Introduction

### Strict Syntax

Rust는 정적 언어이며, 비교적 문법이 엄격한 편이다.

```rust
let an_integer = 1u32;
let a_boolean = true;
let unit = ();

let copied_integer = an_integer;

println!("An integer: {:?}", copied_integer);
println!("A boolean: {:?}", a_boolean);
println!("Meet the unit value: {:?}", unit);
// An integer: 1
// A boolean: true
// Meet the unit value: ()
```

Rust에서 선언된 변수는 기본적으로 불변 (Immutable) 하여 값을 변경할 수 없다. 가변적인 (Mutable) 변수를 선언하려면 `mut` 키워드를 명시해야 한다.

```rust
let immutable_binding = 1;
// immutable_binding += 1; // compiler throw error because immutable by default

let mut mutable_binding = 1;
println!("Before mutation: {}", mutable_binding);
// Before mutation: 1

mutable_binding += 1;
println!("After mutation: {}", mutable_binding);
// After mutation: 2
```

그리고 Rust 컴파일러는 문법이 조금이라도 잘못되면 에러를 낸다. 이러한 엄격함이 언어의 러닝 커브를 높이기도 하지만, 동시에 개발자의 폭주를 막는 역할도 한다. 그래도 그만큼 컴파일러가 꽤나 친절해서 오류가 난 부분과 이유를 상세하게 알려주어, 많은 도움을 받을 수 있다.

### No Null

대부분의 언어에서 Null 값을 Non-Null 값으로 사용하려 할 때 문제가 발생한다. Rust에는 Null이 없다. 대신 표준으로 제공하는 `Option` Enum 멤버로 `None`과 `Some`이 있다.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>`와 `T`는 서로 다른 타입이기 때문에, 컴파일러는 `Option<T>`  값이 무조건 타당한 값인 것처럼 사용하는 것을 거부한다. 예를 들어 아래 코드는 서로 다른 타입인 `i8`과 `Option<i8>`을 더하려고 하기 때문에 컴파일되지 않을 것이다.

```rust
let x: i8 = 1;
let y: Option<i8> = Some(5);

// let sum = x + y; // no implementation for `i8 + std::option::Option<i8>`
```

이는 두 개가 서로 다른 타입이기 때문이다.

```rust
fn plus_one(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,   
        Some(i) => Some(i + 1), 
    }
}

let one: Option<i8> = Some(1);
let two = plus_one(one); // Some(2)
let none = plus_one(None); // None
```

이런식으로 Rust 컴파일러는 항상 수행하려고 하는 연산과 결과가 정당한 지 보장할 수 있게 된다. 그래서 개발자는 그 값이 Null 인지 아닌지 점검할 필요없이 확실을 가지고 사용할 수 있다. 특정 값이 `Option<i8>`을 가지고 있을 경우에는 그 값이 `i8` 값을 가지고 있지 않을 수 있음을 염두에 둬야하고, 컴파일러는 그 값을 사용하기 전에 가능한 경우를 모두 처리하는 지 확실하게 점검한다.

즉, 개발자는 `Options<T>`를 `T`로 바꿔줘야 한다는 것이다. 이러한 작업은 Null로 인해 만연하게 발생하는 문제 (Null인 값을 Null이 아닐 거라고 생각하여 동작하는 로직 등) 를 잡아낸다. 따라서 Null일 수 있는 값을 쓰기 위해서는, 반드시 `Option<T>` 타입을 명시적으로 사용해야만 한다. 그리고 그 값을 사용할 때, Null일 경우를 또한 명시적으로 처리해야만 한다.

### Ownership

Rust는 소유권이라는 방식으로 메모리를 관리한다. C에서는 `malloc`이나 `free` 같은 함수를 이용해 개발자가 직접 메모리를 할당 및 해제한다. Java에서는 GC (Garbage Collector) 가 돌며 메모리를 정리한다. 개발자가 직접 메모리를 관리하면 실수할 위험이 크고, GC를 이용하면 프로그램 성능이 저하될 수 있다. 이 대신 Rust는 소유권이라는 방식으로 메모리를 관리한다.

- 각 값은 Owner라고 불리는 변수를 갖는다.
- 한 번에 하나의 Owner만 가질 수 있다.
- Owner가 범위 (Scope) 를 벗어나면 값이 버려진다.

```rust
let x = 5u32; // _Stack_ allocated integer
let y = x; // Copy `x` into `y` - no resources are moved
println!("x is {}, and y is {}", x, y);
// x is 5, and y is 5
```

```rust
fn create_box() {
    let _box1 = Box::new(3i32); // Allocate an integer on the heap
    // `_box1` is destroyed here, and memory gets freed
}

let _box2 = Box::new(5i32); // Allocate an integer on the heap

{
    let _box3 = Box::new(4i32); // Allocate an integer on the heap
    // `_box3` is destroyed here, and memory gets freed
}

for _ in 0u32..1_000 {
    create_box(); // No need to manually free memory!
}

// `_box2` is destroyed here, and memory gets freed
```

여기서 주의해야할 점이 있다. 두 값 (`s1`, `s2`) 이 같은 Heap 메모리 주소를 가르킬 때, `s1`이 Scope를 벗어났을 때 메모리가 한 번 해제되고, 그 뒤에 `s2`가  Scope를 벗어날 때 같은 메모리 공간을 다시 해제하게 되어, 보안 취약점으로 이어질 수 있다. 그래서 Rust는 할당된 Stack 메모리를 복사할 때 기존에 할당한 변수를 무효화한다.

```rust
fn destroy_box(c: Box<i32>) { // Takes ownership of the heap allocated memory
    println!("Destroying a box that contains {}", c);
    // `c` is destroyed and the memory freed
}

let a = Box::new(5i32); // `a` is a pointer to a _heap_ allocated integer
let b = a; // Move `a` into `b`
// println!("a contains: {}", a); // Error! `a` can no longer access the data

destroy_box(b); // This function takes ownership of the heap allocated memory from `b`

// println!("b contains: {}", b); // Error! Dereference freed memory is forbidden by the compiler
```

### Reference & Borrowing

함수의 인자로 값을 넘기되, 소유권을 이동시키고 싶지 않을 때는 값의 Reference (참조) 만 넘겨주면 된다. 이를 Borrowing (빌림) 이라고 한다.

```rust
fn get_length(s2: &String) -> usize {
    println!("{:?}", s2.as_ptr()); // "0x5581762b0a40"
    s2.len()
}

let s1 = String::from("hello");
let len = get_length(&s1);
println!("{:?}", s1.as_ptr()); // "0x5581762b0a40"

+----------+---+        +----------+---+        +---+---+
| ptr      | ---------->| ptr      | ---------->| 0 | h |
+----------+---+        +----------+---+        +---+---+
       s2               | len      | 5 |        | 1 | e |
                        +----------+---+        +---+---+
                        | capacity | 5 |        | 2 | l |
                        +----------+---+        +---+---+
                               s1               | 3 | l |
                                                +---+---+
                                                | 4 | o |
                                                +---+---+
```

이렇듯, `get_length` 내에서 `s2`의 포인터가 `s1`을 가리키고, `s1`은 Heap 메모리의 문자열 데이터를 가리키게 된다. 함수가 참조만 받았기 때문에 함수 호출 이후에도 `s1`은 유효하다.



## Basic

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

```rust
fn intro() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
    // An integer: 1
    // A boolean: true
    // Meet the unit value: ()
}

fn mutability() {
    let immutable_binding = 1;
    // immutable_binding += 1; // compiler throw error because immutable by default

    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);
    // Before mutation: 1
    // After mutation: 2
}

fn scope() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        // inner short: 2
    }

    // println!("outer short: {}", short_lived_binding); // error! not exist in this scope

    println!("outer long: {}", long_lived_binding);
    // outer long: 1
}

fn shadowing() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);
        // before being shadowed: 1

        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
        // shadowed in inner block: abc
    }

    println!("outside after block: {}", shadowed_binding);
    // outside after block: 1

    let shadowed_binding = 2;
    println!("shadowed after block: {}", shadowed_binding);
    // shadowed after block: 2
}

fn declare_first() {
    let a_binding;
    // println!("before a binding: {}", a_binding); // error! uninitialized variable

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
    // a binding: 4

    let another_binding;
    another_binding = 1;
    println!("another binding: {}", another_binding);
    // another binding: 1
}

fn freezing() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 50; // error! frozen in this scope
    }

    _mutable_integer = 3;
}
```

## Types

```rust
#![allow(overflowing_literals)]

fn casting() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal; // error! no implicit conversion
    let integer = decimal as u8;
    let character = integer as char;
    // let character = decimal as char; // error! cannot be directly converted

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    // Casting: 65.4321 -> 65 -> A

    /*
    when casting any value to an unsigned type, T,
    T::MAX + 1 is added or subtracted until the value
    fits into the new type
     */

    println!("1000 as a u8 is : {}", 1000 as u8); // 1000 - 256 - 256 - 256 = 232
    // 1000 as a u8 is : 232

    println!("  -1 as a u8 is : {}", (-1i8) as u8); // -1 + 256 = 255
    //   -1 as a u8 is : 255


    /*
    When casting to a signed type, the (bitwise) result is the same as
    first casting to the corresponding unsigned type. If the most significant
    bit of that value is 1, then the value is negative.
     */

    println!(" 128 as a i8 is : {}", 128 as i8);
    //  128 as a i8 is : -128

    /*
    Since Rust 1.45, the `as` keyword performs a *saturating cast*
    when casting from float to int. If the floating point value exceeds
    the upper bound or is less than the lower bound, the returned value
    will be equal to the bound crossed.
     */

    println!("300.0 is {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    println!("nan as u8 is {}", f32::NAN as u8);
    // 300.0 is 255
    // -100.0 as u8 is 0
    // nan as u8 is 0

    /*
    This behavior incurs a small runtime cost and can be avoided
    with unsafe methods, however the results might overflow and
    return **unsound values**. Use these methods wisely:
     */
    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
        // 300.0 is 44
        // -100.0 as u8 is 0
        // nan as u8 is 0
    }
}

fn literals() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    // Suffixed literals, their types are known at initialization

    let i = 1;
    let f = 1.0;
    // Unsuffixed literals, their types depend on how they are used

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    // size of `x` in bytes: 1
    // size of `y` in bytes: 4
    // size of `z` in bytes: 4
    // size of `i` in bytes: 4
    // size of `f` in bytes: 8
}

fn inference() {
    let elem = 5u8;

    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

    println!("{:?}", vec);
    // [5]
}

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn aliasing() {
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
    // 5 nanoseconds + 2 inches = 7 unit?
}
```

## Conversion

```rust
use std::convert::From;

#[derive(Debug)]
struct MyNumber {
    value: i32,
}

impl From<i32> for MyNumber {
    fn from(item: i32) -> Self {
        MyNumber { value: item }
    }
}

fn from_and_into() {
    let num_from = MyNumber::from(30);
    println!("num_from is {:?}", num_from);
    // num_from is MyNumber { value: 30 }

    let int = 5;
    let num_to: MyNumber = int.into();
    println!("num_to is {:?}", num_to);
    // num_to is MyNumber { value: 5 }
}

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

impl PartialEq for EvenNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn try_from_and_try_into() {
    assert_eq!(
        EvenNumber::try_from(8),
        Ok(EvenNumber(8))
    );
    assert_eq!(
        EvenNumber::try_from(5),
        Err(())
    );

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(
        result,
        Ok(EvenNumber(8))
    );
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(
        result,
        Err(())
    );
}

use std::fmt;
use std::fmt::Formatter;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn to_and_from_strings() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    // Circle of radius 6

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
    // Sum: 15
}
```

## Expressions

```rust
fn expressions() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
        // This no semicolon expression will be assigned to `y`
    };

    let z = {
        2 * x;
        // The semicolon suppresses this expression and `()` is assigned to `z`
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    // x is 5
    // y is 155
    // z is ()
}
```

## Flow of Control

```rust
fn if_else() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);
    // 5 -> 50
}

fn loop_and_break() {
    let mut count = 0u32;

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
    // 1
    // 2
    // three
    // 4
    // 5
    // OK, that's enough
}

fn nesting_and_labels() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
    // Entered the outer loop
    // Entered the inner loop
    // Exited the outer loop
}

fn returning_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn while_loops() {
    let mut n = 1;

    while n < 16 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }
    // 1
    // 2
    // fizz
    // 4
    // buzz
    // fizz
    // 7
    // 8
    // fizz
    // buzz
    // 11
    // fizz
    // 13
    // 14
    // fizzbuzz
}

fn for_loops() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() { // borrow each element
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names(iter): {:?}", names);
    // Hello Bob
    // Hello Frank
    // There is a rustacean among us!
    // names(iter): ["Bob", "Frank", "Ferris"]

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() { // consume collection
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names(into_iter): {:?}", names); // error! `names` collection is no longer available
    // Hello Bob
    // Hello Frank
    // There is a rustacean among us!

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {  // mutably borrow element
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names(iter_mut): {:?}", names);
    // names(iter_mut): ["Hello", "Hello", "There is a rustacean among us!"]
}

fn match_keyword() {
    let number = 13;

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        10..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    // This is a prime

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
    // true -> 1
}

fn match_destructuring() {
    /*
    tuples
     */
    let triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        _      => println!("It doesn't matter what they are"),
    }
    // First is `0`, `y` is -2, and `z` is 3

    /*
    arrays/slices
     */
    let array = [-1, 2, -3];
    match array {
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] =>
            println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
        [2, second, ..] =>
            println!("array[0] = 2, array[1] = {} and all the other ones were ignored", second),
        [3, second, tail @ ..] =>
            println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        [first, middle @ .., last] =>
            println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }
    // array[0] = -1, middle = [2], array[2] = -3

    /*
    enums
     */
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red   =>
            println!("The color is Red!"),
        Color::Blue  =>
            println!("The color is Blue!"),
        Color::Green =>
            println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }
    // Red: 122, green: 17, and blue: 40!

    /*
    pointers/ref
    - Dereferencing uses *
    - Destructuring uses &, ref, and ref mut
     */
    let reference = &1;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    // Got a value via destructuring: 1
    // Got a value via dereferencing: 1

    let value = 2;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    // Got a reference to a value: 2

    let mut mut_value = 3;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
    // We added 10. `mut_value`: 13\

    /*
    structs
     */
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (0, 1), y: 2 };
    match foo {
        Foo { x: (1, b), y } =>
            println!("First of x is 1, b = {}, y = {} ", b, y),
        Foo { y: 2, x: i } =>
            println!("y is 2, i = {:?}", i),
        Foo { y, .. } =>
            println!("y = {}, we don't care about x", y),
    }
    // y is 2, i = (0, 1)
}

fn match_guards() {
    let pair = (2, -2);
    match pair {
        (x, y) if x == y =>
            println!("These are twins"),
        (x, y) if x + y == 0 =>
            println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 =>
            println!("The first one is odd"),
        _ =>
            println!("No correlation..."),
    }
    // Antimatter, kaboom!

    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"), // does not check arbitrary expressions
    }
    // Greater than zero
}

fn match_binding() {
    fn age() -> u32 {
        19
    }
    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n             => println!("I'm an old person of age {:?}", n),
    }
    // I'm a teen of age 19

    fn some_number() -> Option<u32> {
        Some(42)
    }
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
    // The Answer: 42!
}

fn if_let() {
    let number = Some(7);
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
    // Matched 7!

    let letter: Option<i32> = None;
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
    // Didn't match a number. Let's go with a letter!

    let emoticon: Option<i32> = None;
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
    // I don't like letters. Let's go with an emoticon :)!

    enum Foo {
        Bar,
        Qux(u32),
    }

    let a = Foo::Bar;
    // if Foo::Bar == a { // enum purposely neither implements nor derives PartialEq
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    // a is foobar

    let b = Foo::Qux(100);
    if let Foo::Qux(value @ 100) = b {
        println!("b is one hundred");
    }
    // b is one hundred
}

fn while_let() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 5 {
            println!("Greater than 5, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
    // `i` is `0`. Try again.
    // `i` is `1`. Try again.
    // `i` is `2`. Try again.
    // `i` is `3`. Try again.
    // `i` is `4`. Try again.
    // `i` is `5`. Try again.
    // Greater than 5, quit!
}
```

## Functions

```rust
fn intro() {
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }

    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
        }
    }

    fizzbuzz_to(16);
    // 1
    // 2
    // fizz
    // 4
    // buzz
    // fizz
    // 7
    // 8
    // fizz
    // buzz
    // 11
    // fizz
    // 13
    // 14
    // fizzbuzz
    // 16
}

fn associated_functions_and_methods() {
    struct Point {
        x: f64,
        y: f64,
    }
    impl Point {
        /*
        These are an "associated functions" because they are
        associated with a particular type, that is, Point.
        Generally used like constructors.
         */
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    impl Rectangle {
        /*
        This is a method.
        `&self` is sugar for `self: &Self`, where `Self` is the type
        of the caller object. In this case `Self` = `Rectangle`.
         */
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            ((x1 - x2) * (y1 - y2)).abs()
        }
        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p1.y += y;
            self.p2.x += x;
            self.p2.y += y;
        }
    }

    /*
    Associated functions are called using double colons
     */
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    /*
    Methods are called using the dot operator
    Note that the first argument `&self` is implicitly passed.
     */
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    // rectangle.translate(1.0, 0.0); // error! `rectangle` is immutable
    square.translate(1.0, 1.0);
    // Rectangle perimeter: 14
    // Rectangle area: 12

    struct Pair(Box<i32>, Box<i32>); // `Pair` owns resources: two heap allocated integers
    impl Pair {
        /*
        This method "consumes" the resources of the caller object
        when go out of scope and get freed.
        `self` desugars to `self: Self`.
         */
        fn destroy(self) {
            let Pair(first, second) = self;
            println!("Destroying Pair({}, {})", first, second);
        }
    }

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // Destroying Pair(1, 2)
}

fn closures() {
    /*
    Closures are anonymous, here we are binding them to references.
    These nameless functions are assigned to appropriately named variables.
    - using || instead of () around input variables.
    - optional body delimination ({}) for a single expression (mandatory otherwise).
    - the ability to capture the outer environment variables.
     */
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    // closure_annotated: 2
    // closure_inferred: 2

    let one = || 1;
    println!("closure returning one: {}", one());
    // closure returning one: 1
}

fn closures_capturing() {
    /*
    A closure to print `color` which immediately borrows (`&`) `color` and
    stores the borrow and closure in the `print` variable.
    `println!` only requires arguments by immutable reference so it doesn't
    impose anything more restrictive.
     */
    let color = String::from("green");
    let print = || println!("`color`: {}", color);

    print();
    // `color`: green

    let _reborrow = &color; // `color` can be borrowed immutable again.
    print();
    // `color`: green

    let _color_moved = color;
    // print(); // error! `color` is moved out

    /*
    A closure to increment `count` could take either `&mut count` or `count`
    but `&mut count` is less restrictive so it takes that. Immediately
    borrows `count`.
     */
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    // `count`: 1

    // let _reborrow = &count; // error! closure `inc` is called later
    inc();
    // `count`: 2

    let _count_reborrowed = &mut count; // closure `inc` no longer needs to borrorw `&mut count`
}

fn closures_move() {
    let vec1 = vec![1, 2, 3]; // `Vec` has non-copy semantics.
    let contains_move = move |value| vec1.contains(value);

    println!("{}", contains_move(&1));
    println!("{}", contains_move(&4));
    // println!("{} elements in vec", vec1.len()); // error! cannot re-use variable which has been moved
    // true
    // false

    let vec2 = vec![1, 2, 3]; // `Vec` has non-copy semantics.
    let contains_non_move = |value| vec2.contains(value);

    println!("{}", contains_non_move(&1));
    println!("{}", contains_non_move(&4));
    println!("{} elements in vec", vec2.len());
    // true
    // false
    // 3 elements in vec
}

fn closures_as_input_params() {
    /*
    In order of decreasing restriction, closure's traits are:
    - Fn: the closure uses the captured value by reference (&T)
    - FnMut: the closure uses the captured value by mutable reference (&mut T)
    - FnOnce: the closure uses the captured value by value (T)
     */

    fn apply<F>(f: F)
        where F: FnOnce() {
        f();
    }

    let greeting = "hello"; // a non-copy type
    let mut farewell = "goodbye".to_owned(); // create owned data from borrowed one

    let diary = || {
        println!("I said {}.", greeting); // `greeting` is by reference: requires `Fn`.
        // I said hello.

        farewell.push_str("!!!"); // requires `f` of closure `apply` is FnMut.
        println!("Then I screamed {}.", farewell); // `farewell` is by mutable reference: requires `FnMut`.
        // Then I screamed goodbye!!!.

        std::mem::drop(farewell); // requires `f` of closure `apply` is FnOnce.
    };
    apply(diary);

    fn apply_to_3<F>(f: F) -> i32
        where F: Fn(i32) -> i32 {
        f(3)
    }

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
    // 3 doubled: 6
}

fn closures_type_anonymity() {
    fn apply<F>(f: F)
        where F: Fn() {
        f();
    }

    let x = 7;
    let print = || println!("{}", x); // Capture `x` into an anonymous type and implement `Fn` for it.

    apply(print);
    // 7
}

fn closures_input_functions() {
    fn call_me<F: Fn()>(f: F) {
        f();
    }
    fn my_func() {
        println!("I am a function");
    }

    let closure = || println!("I am a closure");

    call_me(closure);
    call_me(my_func);
    // I am a closure
    // I am a function
}

fn closures_as_output_params() {
    /*
    Anonymous closure types are unknown, so we have to use `impl Trait` to return them.
    The valid traits for returning a closure are:
    - Fn
    - FnMut
    - FnOnce
    Beyond this, the move keyword must be used, which signals that all captures occur by value.
    This is required because any captures by reference would be dropped as soon as the function exited,
    leaving invalid references in the closure.
     */

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a: {}", text)
    }
    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a: {}", text)
    }
    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
    // This is a: Fn
    // This is a: FnMut
    // This is a: FnOnce
}

fn closures_iterator_any() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2)); // `iter()` for vec yields `&i32`. Destructure to `i32`.
    println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2)); // `into_iter()` for vec yields `i32`. No destructuring required.
    // 2 in vec1: true
    // 2 in vec2: false

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2)); // `iter()` for array yields `&i32`.
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2)); // `into_iter()` for array yields `i32`.
    // 2 in array1: true
    // 2 in array2: false
}

fn higher_order_functions() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    let upper = 1000;

    /*
    Imperative approach
     */
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);
    // imperative style: 5456

    /*
    Functional approach
     */
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
            .take_while(|&n_squared| n_squared < upper)
            .filter(|&n_squared| is_odd(n_squared))
            .fold(0, |acc, n_squared| acc + n_squared);
    println!("functional style: {}", sum_of_squared_odd_numbers);
    // functional style: 5456
}

fn diverging_functions() {
    /*
    Diverging functions never return. They are marked using `!`, which is an empty type.
     */

    fn foo() -> ! {
        panic!("This call never returns.");
    }

    /*
    Usage of diverging concept, `match` branches.
     */
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32
            // because of the type of the "addition" variable.
            let addition: u32 = match i%2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine.
                true => i,
                // On the other hand, the "continue" expression does not return
                // u32, but it is still fine, because it never returns and therefore
                // does not violate the type requirements of the match expression.
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
    // Sum of odd numbers up to 9 (excluding): 16
}
```

## Modules

```rust
mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }
    pub fn function() {
        println!("called `my_mod::function()`");
    }
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }
        pub(self) fn public_function_in_nested() { // the same as leaving them private
            println!("called `my_mod::nested::public_function_in_nested()`");
        }
        pub(in crate::j_modules::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn visibility() {
    my_mod::function();
    // called `my_mod::function()`

    /*
    Public items, including those inside nested modules,
    can be accessed from outside the parent module.
     */
    my_mod::indirect_access();
    // called `my_mod::indirect_access()`, that
    // > called `my_mod::private_function()`

    my_mod::nested::function();
    // called `my_mod::nested::function()`

    my_mod::call_public_function_in_my_mod();
    // called `my_mod::call_public_function_in_my_mod()`, that
    // > called `my_mod::nested::public_function_in_my_mod()`, that
    // > called `my_mod::nested::public_function_in_nested()`
    // > called `my_mod::nested::public_function_in_super_mod()`

    /*
    pub(crate) items can be called from anywhere in the same crate
     */
    my_mod::public_function_in_crate();
    // called `my_mod::nested::function()`

    /*
    pub(in path) items
     */
    // my_mod::nested::public_function_in_my_mod(); // error! `public_function_in_my_mod` can only be called from within the module specified
    // my_mod::private_nested::function(); // error! 'private_nested` is a private module
    // my_mod::private_nested::restricted_function(); // error! `private_nested` is a private module
}

mod my_box {
    pub struct OpenBox<T> {
        pub contents: T,
    }
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(c: T) -> ClosedBox<T> {
            ClosedBox {
                contents: c,
            }
        }
    }
}

fn struct_visibility() {
    let open_box = my_box::OpenBox {
        contents: "public information"
    };
    println!("The open box contains: {}", open_box.contents);
    // The open box contains: public information

    // let closed_box = my_box::ClosedBox { contents: "classified information" }; // error! `ClosedBox` has private fields
    let _closed_box = my_box::ClosedBox::new("classified information");
    // println!("The closed box contains: {}", _closed_box.contents); // error! `contents` field is private
}

use crate::j_modules::deeply::nested::function as other_function;

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn use_declaration() {
    other_function();
    // called `deeply::nested::function()`

    fn function() {
        println!("called `function()`");
    }

    println!("Entering block");
    {
        use crate::j_modules::deeply::nested::function;

        function(); // `use` bindings have a local scope. The shadowing of `function()` is only in this block.
        println!("Leaving block");
    }
    function();
    // Entering block
    // called `deeply::nested::function()`
    // Leaving block
    // called `function()`
}

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");

        self::function();
        function();
        self::cool::function();
        super::function();
        {
            use crate::j_modules::cool::function as root_function;
            root_function();
        }
    }
}

fn super_and_self() {
    my::indirect_call();
    // called `my::indirect_call()`, that
    // > called `my::function()`
    // called `my::function()`
    // called `my::cool::function()`
    // called `function()`
    // called `cool::function()`
}
```

## Crates

```rust
fn intro() {
    /*
    A crate is a compilation unit in Rust.
    Whenever rustc some_file.rs is called, some_file.rs is treated as the crate file.
     */
}
```

## Cargo

```rust
fn intro() {
    /*
    `cargo` is the official Rust package management tool.
    - Dependency management and integration with `crates.io` (the official Rust package registry)
    - Awareness of unit tests
    - Awareness of benchmarks
     */
}

fn dependencies() {
    /*
    To create a new Rust project,
    - `cargo new foo` for a binary
    - `cargo new --lib foo` for a library
     */
}

fn conventions() {
    /*
    Suppose that we wanted to have two binaries in the same project.
    You can add additional binaries by placing them in a `bin/` directory:
    ---
    foo
    ├── Cargo.toml
    └── src
        ├── main.rs
        └── bin
            └── my_other_bin.rs
    ---
     */
}

fn testing() {
    /*
    Organizationally, we can place unit tests in the modules
    they test and integration tests in their own tests/ directory:
    ---
    foo
    ├── Cargo.toml
    ├── src
    │   └── main.rs
    │   └── lib.rs
    └── tests
        ├── my_test.rs
        └── my_other_test.rs
    ---
     */
}
```

## Attributes

```rust
fn intro() {
    /*
    An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
    - conditional compilation of code
    - set crate name, version and type (binary or library)
    - disable lints (warnings)
    - enable compiler features (macros, glob imports, etc.)
    - link to a foreign library
    - mark functions as unit tests
    - mark functions that will be part of a benchmark

    Attributes can take arguments with different syntaxes:
    - #[attribute = "value"]
    - #[attribute(key = "value")]
    - #[attribute(value)]
     */
}

fn dead_code() {
    fn used_function() {}

    #[allow(dead_code)]
    fn unused_function() {}

    used_function();
}

fn cfg() {
    /*
    Configuration conditional checks are possible through two different operators:
    - the cfg attribute: `#[cfg(...)]` in attribute position
    - the cfg! macro: `cfg!(...)` in boolean expressions
     */

    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!");
    }
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!");
    }

    are_you_on_linux();
    // You are *not* running linux!

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
    // Are you sure?
    // Yes. It's definitely *not* linux!
}
```

## Generics

```rust
fn generics_functions() {
    struct A;          // Concrete type `A`.
    struct S(A);       // Concrete type `S`.
    struct SGen<T>(T); // Generic type `SGen`.

    /*
    Define a function `reg_fn` that takes an argument `_s` of type `S`.
    This has no `<T>` so this is not a generic function.
     */
    fn reg_fn(_s: S) {}

    /*
    Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
    It has been explicitly given the type parameter `A`, but because `A` has not
    been specified as a generic type parameter for `gen_spec_t`, it is not generic.
     */
    fn gen_spec_t(_s: SGen<A>) {}

    /*
    Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
    Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
     */
    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A)); // Concrete type.
    gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.

    generic::<char>(SGen('a')); // Explicitly specified type parameter `char` to `generic()`.
    generic(SGen('b')); // Implicitly specified type parameter `char` to `generic()`.
}

fn generics_implementations() {
    struct Val {
        val: f64,
    }
    struct GenVal<T> {
        gen_val: T,
    }
    impl Val { // impl of `Val`
        fn value(&self) -> &f64 {
            &self.val
        }
    }
    impl<T> GenVal<T> { // impl of `GenVal` for a generic type `T`
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
    // 3, 3
}

fn generics_traits() {
    struct Empty; // Non-copyable types.
    struct Null; // Non-copyable types.

    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }
    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {} // This method takes ownership of both passed arguments, deallocating both.
    }

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}

fn generics_bounds() {
    use std::fmt::Debug;

    #[derive(Debug)]
    struct Rectangle { length: f64, height: f64 }
    #[allow(dead_code)]
    struct Triangle  { length: f64, height: f64 }

    trait HasArea {
        fn area(&self) -> f64;
    }
    impl HasArea for Rectangle {
        fn area(&self) -> f64 { self.length * self.height }
    }

    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());
    // Rectangle { length: 3.0, height: 4.0 }
    // Area: 12

    let _triangle = Triangle { length: 3.0, height: 4.0 };
    // print_debug(&_triangle); // error! Does not implement either `Debug`.
    // println!("Area: {}", _triangle.area()); // error! Does not implement either `HasArea`.
}

fn generics_multiple_bounds() {
    use std::fmt::{
        Debug,
        Display,
    };

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }
    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_types(&array, &vec);
    // t: `[1, 2, 3]`
    // u: `[1, 2, 3]`

    let string = "words";
    compare_prints(&string);
    // Debug: `"words"`
    // Display: `words`
}

use std::fmt::Debug;

fn generics_where_clauses() {
    /*
    When specifying generic types and bounds separately is clearer
     */
    struct YourType {}
    trait MyTrait<T, U> {}
    trait TraitB {}
    trait TraitC {}
    trait TraitE {}
    trait TraitF {}

    impl <A, D> MyTrait<A, D> for YourType where
        A: TraitB + TraitC,
        D: TraitE + TraitF {
    }

    /*
    When using a `where` clause is more expressive than using normal syntax.
    The `impl` in this example cannot be directly expressed without a `where` clause
     */
    trait PrintInOption {
        fn print_in_option(self);
    }

    impl<T> PrintInOption for T where
        Option<T>: Debug {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    vec.print_in_option();
    // Some([1, 2, 3])
}

fn generics_new_type_idiom() {
    struct Years(i64);
    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    struct Days(i64);
    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // Old enough false
    // Old enough false
}

fn generics_associated_items() {
    struct Container(i32, i32);

    /*
    Previous:
    ---
    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
        fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
        fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
    }
    ---
     */
    trait Contains {
        type A;
        type B;
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    /*
    Previous:
    ---
    impl Contains<i32, i32> for Container {
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        fn first(&self) -> i32 { self.0 }
        fn last(&self) -> i32 { self.1 }
    }
    ---
     */
    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    /*
    Previous:
    ---
    fn difference<A, B, C>(container: &C) -> i32 where
        C: Contains<A, B> {
        container.last() - container.first()
    }
    ---
     */
    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));
    // Does container contain 3 and 10: true
    // First number: 3
    // Last number: 10
    // The difference is: 7
}

use std::marker::PhantomData;

fn generics_phantom_type_params() {
    /*
    A phantom type parameter is one that doesn't show up at runtime,
    but is checked statically (and only) at compile time.
     */

    /*
    A phantom tuple struct which is generic over `A` with hidden parameter `B`.
     */
    #[derive(PartialEq)]
    struct PhantomTuple<A, B>(A, PhantomData<B>);

    /*
    A phantom type struct which is generic over `A` with hidden parameter `B`.
     */
    #[derive(PartialEq)]
    struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // println!(
    //     "_tuple1 == _tuple2 yields: {}",
    //     _tuple1 == _tuple2
    // ); // Compile-time error! Type mismatch so these cannot be compared:

    // println!(
    //     "_struct1 == _struct2 yields: {}",
    //     _struct1 == _struct2
    // ); // Compile-time Error! Type mismatch so these cannot be compared:
}
```

## Scoping Rules

```rust
fn intro() {
    /*
    Scopes play an important part in ownership, borrowing, and lifetimes.
    They indicate to the compiler
    - when borrows are valid
    - when resources can be freed
    - when variables are created or destroyed.
     */
}

fn raii() {
    /*
    Variables in Rust do more than just hold data in the stack: they also own resources.
    Rust enforces RAII (Resource Acquisition Is Initialization),
    so whenever an object goes out of scope, its destructor is called and
    its owned resources are freed.
     */

    fn create_box() {
        let _box1 = Box::new(3i32); // Allocate an integer on the heap

        // `_box1` is destroyed here, and memory gets freed
    }

    let _box2 = Box::new(5i32); // Allocate an integer on the heap
    {
        let _box3 = Box::new(4i32); // Allocate an integer on the heap

        // `_box3` is destroyed here, and memory gets freed
    }

    for _ in 0u32..1_000 {
        create_box(); // No need to manually free memory!
    }

    // `_box2` is destroyed here, and memory gets freed
}

fn raii_destructor() {
    struct ToDrop;
    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    let x = ToDrop;
    println!("Made a ToDrop");
    println!("`raii_destructor` function is finished");
    // Made a ToDrop
    // `raii_destructor` function is finished
    // ToDrop is being dropped
}

fn ownership_and_moves() {
    /*
    Because variables are in charge of freeing their own resources,
    resources can only have one owner. This also prevents resources
    from being freed more than once.
     */

    fn destroy_box(c: Box<i32>) { // Takes ownership of the heap allocated memory
        println!("Destroying a box that contains {}", c);

        // `c` is destroyed and the memory freed
    }

    let x = 5u32; // _Stack_ allocated integer
    let y = x; // Copy `x` into `y` - no resources are moved
    println!("x is {}, and y is {}", x, y);
    // x is 5, and y is 5

    /*
    The pointer address of `a` is copied (not the data) into `b`.
    Both are now pointers to the same heap allocated data, but
    `b` now owns it.
     */
    let a = Box::new(5i32); // `a` is a pointer to a _heap_ allocated integer
    let b = a; // Move `a` into `b`
    // println!("a contains: {}", a); // Error! `a` can no longer access the data

    destroy_box(b); // This function takes ownership of the heap allocated memory from `b`
    // Destroying a box that contains 5

    // println!("b contains: {}", b); // Error! Dereference freed memory is forbidden by the compiler
}

fn ownership_and_movers_of_mutability() {
    /*
    Mutability of data can be changed when ownership is transferred.
     */

    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
    // immutable_box contains 5

    // *immutable_box = 4; // Mutability error

    let mut mutable_box = immutable_box; // Move the box, changing the ownership (and mutability)
    println!("mutable_box contains {}", mutable_box);
    // mutable_box contains 5

    *mutable_box = 4; // Modify the contents of the box
    println!("mutable_box now contains {}", mutable_box);
    // mutable_box now contains 4
}

fn partial_moves() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>, // Store the `age` on the heap to illustrate the partial move
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person {
        name,
        ref age
    } = person; // `name` is moved out of person, but `age` is referenced
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    // The person's age is 20
    // The person's name is Alice

    // println!("The person struct is {:?}", person); // Error! borrow of partially moved value: `person` partial move occurs
    println!("The person's age from person struct is {}", person.age); // `person` cannot be used but `person.age` can be used as it is not moved
    // The person's age from person struct is 20
}

fn borrowing() {
    /*
    Most of the time, we'd like to access data without taking ownership over it.
    To accomplish this, Rust uses a borrowing mechanism. Instead of passing objects
    by value (T), objects can be passed by reference (&T).

    The compiler statically guarantees (via its borrow checker) that references
    always point to valid objects. That is, while references to an object exist,
    the object cannot be destroyed.
     */

    fn eat_box_i32(boxed_i32: Box<i32>) { // Takes ownership of a box and destroys it
        println!("Destroying box that contains {}", boxed_i32);
    }
    fn borrow_i32(borrowed_i32: &i32) { // Borrows an i32
        println!("This int is: {}", borrowed_i32);
    }

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32); // Ownership is not taken, so the contents can be borrowed again.
    borrow_i32(&stacked_i32); // Ownership is not taken, so the contents can be borrowed again.
    // This int is: 5
    // This int is: 6

    {
        let _ref_to_i32: &i32 = &boxed_i32;

        // eat_box_i32(boxed_i32); // Error! Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        borrow_i32(_ref_to_i32); // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        // This int is: 5
    }

    eat_box_i32(boxed_i32); // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    // Destroying box that contains 5
}

fn borrowing_mutability() {
    /*
    Mutable data can be mutably borrowed using `&mut T`. This is called
    a mutable reference and gives read/write access to the borrower.
    In contrast, `&T` borrows the data via an immutable reference,
    and the borrower can read the data but not modify it
     */

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        author: &'static str, // `&'static str` is a reference to a string allocated in read only memory
        title: &'static str, // `&'static str` is a reference to a string allocated in read only memory
        year: u32,
    }

    fn borrow_book(book: &Book) { // Takes a reference to a book
        println!("I immutably borrowed {} - {} edition", book.title, book.year);
    }
    fn new_edition(book: &mut Book) { // Takes a reference to a mutable book and changes `year` to 2014
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };
    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);
    // I immutably borrowed Gödel, Escher, Bach - 1979 edition
    // I immutably borrowed Gödel, Escher, Bach - 1979 edition

    // new_edition(&mut immutabook); // Error! Cannot borrow an immutable object as mutable
    new_edition(&mut mutabook);
    // I mutably borrowed Gödel, Escher, Bach - 2014 edition
}

fn borrowing_aliasing() {
    /*
    Data can be immutably borrowed any number of times, but while immutably borrowed,
    the original data can't be mutably borrowed. On the other hand, only one mutable borrow
    is allowed at a time. The original data can be borrowed again only after the mutable
    reference has been used for the last time.
     */

    struct Point { x: i32, y: i32, z: i32 }

    let mut point = Point { x: 0, y: 0, z: 0 };
    let borrowed_point = &point;
    let another_borrow = &point;
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    ); // Point has coordinates: (0, 0, 0)

    // let mutable_borrow = &mut point; // Error! Can't borrow `point` as mutable because it's currently borrowed as immutable.
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    ); // Point has coordinates: (0, 0, 0)

    let mutable_borrow = &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // let y = &point.y; // Error! Can't borrow `point` as immutable because it's currently borrowed as mutable.
    // println!("Point Z coordinate is {}", point.z); // Error! Can't print because `println!` takes an immutable reference.
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    ); // Point has coordinates: (5, 2, 1)

    let new_borrowed_point = &point;
    println!(
        "Point now has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    ); // Point now has coordinates: (5, 2, 1)
}

fn borrowing_the_ref_pattern() {
    /*
    A `ref` borrow on the left side of an assignment is equivalent to
    an `&` borrow on the right side.
     */
    let c = 'Q';
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    // ref_c1 equals ref_c2: true

    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
    // tuple is (5, 2)

    #[derive(Clone, Copy)]
    struct Point { x: i32, y: i32 }

    let point = Point { x: 0, y: 0 };
    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _
        } = point;

        *ref_to_x // Return a copy of the `x` field of `point`.
    };

    let mut mutable_point = point;
    {
        let Point {
            x: _,
            y: ref mut mut_ref_to_y
        } = mutable_point;

        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);
    // point is (0, 0)
    // mutable_point is (0, 1)
}

fn lifetime() {
    /*
    Lifetimes are annotated below with lines denoting the creation
    and destruction of each variable. `i` has the longest lifetime
    because its scope entirely encloses both `borrow1` and `borrow2`.
    The duration of `borrow1` compared to `borrow2` is irrelevant
    since they are disjoint.
     */

    let i = 3; // Lifetime for `i` starts. ─────────────────┐
    //                                                           │
    { //                                                         │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                      ││
        println!("borrow1: {}", borrow1); //                    ││
    } // `borrow1 ends. ────────────────────────────────────────┘│
    { //                                                         │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                      ││
        println!("borrow2: {}", borrow2); //                    ││
    } // `borrow2` ends. ───────────────────────────────────────┘│
    //                                                           │
}   // Lifetime ends. ───────────────────────────────────────────┘

fn lifetime_explicit_annotation() {
    /*
    The borrow checker uses explicit lifetime annotations to determine
    how long references should be valid. In cases where lifetimes are
    not elided, Rust requires explicit annotations to determine what
    the lifetime of a reference should be.

    Similar to closures, using lifetimes requires generics. Additionally,
    this lifetime syntax indicates that the lifetime of foo may not exceed
    that of 'a. Explicit annotation of a type has the form `&'a T` where
    'a has already been introduced.

    ---
    foo<'a> // `foo` has a lifetime parameter `'a`
    ---
     */

    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        /*
        `print_refs` takes two references to `i32` which have different
        lifetimes `'a` and `'b`. These two lifetimes must both be at
        least as long as the function `print_refs`.
         */
        println!("x is {} and y is {}", x, y);
    }
    fn failed_borrow<'a>() { // Takes no arguments, but has a lifetime parameter `'a`.
        /*
        Attempting to use the lifetime `'a` as an explicit type annotation
        inside the function will fail because the lifetime of `&_x` is shorter
        than that of `y`. A short lifetime cannot be coerced into a longer one.
         */
        let _x = 12;
        // let y: &'a i32 = &_x; // ERROR: `_x` does not live long enough
    }

    /*
    Any input which is borrowed must outlive the borrower.
    In other words, the lifetime of `four` and `nine` must
    be longer than that of `print_refs`.
     */
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    // x is 4 and y is 9

    /*
    `failed_borrow` contains no references to force `'a` to be
    longer than the lifetime of the function, but `'a` is longer.
    Because the lifetime is never constrained, it defaults to `'static`.
     */
    failed_borrow();
}

fn lifetime_functions() {
    /*
    Ignoring elision, function signatures with lifetimes have a few constraints:
    - any reference must have an annotated lifetime.
    - any reference being returned must have the same lifetime as an input or be static.
     */

    /*
    One input reference with lifetime `'a` which must live at least as long as the function.
     */
    fn print_one<'a>(x: &'a i32) {
        println!("`print_one`: x is {}", x);
    }

    /*
    Mutable references are possible with lifetimes as well.
     */
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    /*
    Multiple elements with different lifetimes. In this case, it
    would be fine for both to have the same lifetime `'a`, but
    in more complex cases, different lifetimes may be required.
     */
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("`print_multi`: x is {}, y is {}", x, y);
    }

    /*
    Returning references that have been passed in is acceptable.
    However, the correct lifetime must be returned.
     */
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

    /*
    The below is invalid: `'a` must live longer than the function.
    Here, `&String::from("foo")` would create a `String`, followed by a
    reference. Then the data is dropped upon exiting the scope, leaving
    a reference to invalid data to be returned.
     */
    // fn invalid_output<'a>() -> &'a String {
    //     &String::from("foo")
    // }

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);
    // `print_one`: x is 7
    // `print_multi`: x is 7, y is 9

    let z = pass_x(&x, &y);
    print_one(z);
    // `print_one`: x is 7

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
    // `print_one`: x is 4
}

fn lifetime_methods() {
    struct Owner(i32);
    impl Owner {
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }
        fn print<'a>(&'a self) {
            println!("`print`: {}", self.0);
        }
    }

    let mut owner = Owner(18);
    owner.add_one();
    owner.print();
    // `print`: 19
}

fn lifetime_structs() {
    /*
    A type `Borrowed` which houses a reference to an `i32`.
    The reference to `i32` must outlive `Borrowed`.
     */
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    /*
    Similarly, both references here must outlive this structure.
     */
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    /*
    An enum which is either an `i32` or a reference to one.
     */
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
    // x is borrowed in Borrowed(18)
    // x and y are borrowed in NamedBorrowed { x: 18, y: 15 }
    // x is borrowed in Ref(18)
    // y is *not* borrowed in Num(15)
}

fn lifetime_traits() {
    #[derive(Debug)]
    struct Borrowed<'a> { x: &'a i32 }
    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            Self { x: &10 }
        }
    }

    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
    // b is Borrowed { x: 10 }
}

fn lifetime_bounds() {
    /*
    Just like generic types can be bounded, lifetimes (themselves generic)
    use bounds as well. The : character has a slightly different meaning here,
    but + is the same.
    - T: 'a: All references in T must outlive lifetime 'a.
    - T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a.
     */

    use std::fmt::Debug;

    /*
    `Ref` contains a reference to a generic type `T` that has an unknown
    lifetime `'a`. `T` is bounded such that any *references* in `T` must
    outlive `'a`. Additionally, the lifetime of `Ref` may not exceed `'a`.
     */
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    fn print<T>(t: T) where
        T: Debug {
        println!("`print`: t is {:?}", t);
    }

    /*
    Here a reference to `T` is taken where `T` implements `Debug`and all
    *references* in `T` outlive `'a`. In addition, `'a` must outlive the function.
     */
    fn print_ref<'a, T>(t: &'a T) where
        T: Debug + 'a {
        println!("`print_ref`: t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
    // `print_ref`: t is Ref(7)
    // `print`: t is Ref(7)
}

fn lifetime_coercion() {
    /*
    A longer lifetime can be coerced into a shorter one so that it works
    inside a scope it normally wouldn't work in. This comes in the form
    of inferred coercion by the Rust compiler, and also in the form of
    declaring a lifetime difference.
     */

    /*
    Here, Rust infers a lifetime that is as short as possible.
    The two references are then coerced to that lifetime.
     */
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    /*
    `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
    Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
     */
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    let first = 2; // Longer lifetime
    {
        let second = 3; // Shorter lifetime

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
        // The product is 6
        // 2 is the first
    };
}

fn lifetime_static() {
    /*
    REFERENCE LIFETIME

    As a reference lifetime 'static indicates that the data pointed to by the reference
    lives for the entire lifetime of the running program. It can still be coerced to
    a shorter lifetime. There are two ways to make a variable with 'static lifetime,
    and both are stored in the read-only memory of the binary:
    - Make a constant with the static declaration.
    - Make a string literal which has type: &'static str.
     */

    static NUM: i32 = 18;
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 { // lifetime is coerced to that of the input argument.
        &NUM
    }

    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
        // static_string: I'm in read-only memory

        /*
        When `static_string` goes out of scope, the reference
        can no longer be used, but the data remains in the binary.
         */
    }
    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num); // Coerce `NUM` to lifetime of `lifetime_num`:
        println!("coerced_static: {}", coerced_static);
        // coerced_static: 18
    }

    println!("NUM: {} stays accessible!", NUM);
    // NUM: 18 stays accessible!

    /*
    TRAIT BOUND

    As a trait bound, it means the type does not contain any non-static references.
    Eg. the receiver can hold on to the type for as long as they want and it will
    never become invalid until they drop it. It's important to understand this means
    that any owned data always passes a 'static lifetime bound, but a reference to
    that owned data generally does not
     */

    use std::fmt::Debug;

    fn print_it( input: impl Debug + 'static ) {
        println!( "'static value passed in is: {:?}", input);
    }

    let i = 5;
    // print_it(&i); // error! &i only has the lifetime defined by the scope of main()
    print_it(i); // i is owned and contains no references, thus it's 'static
    // 'static value passed in is: 5
}

fn lifetime_elision() {
    /*
    Some lifetime patterns are overwhelmingly common and so the borrow checker
    will allow you to omit them to save typing and to improve readability.
    This is known as elision. Elision exists in Rust solely because these patterns are common.
     */

    /*
    `elided_input` and `annotated_input` essentially have identical signatures
    because the lifetime of `elided_input` is inferred by the compiler:
     */
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x);
    }
    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x);
    }

    /*
    Similarly, `elided_pass` and `annotated_pass` have identical signatures
    because the lifetime is added implicitly to `elided_pass`:
     */
    fn elided_pass(x: &i32) -> &i32 { x }
    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

    let x = 3;

    elided_input(&x);
    annotated_input(&x);
    // `elided_input`: 3
    // `annotated_input`: 3

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
    // `elided_pass`: 3
    // `annotated_pass`: 3
}
```

## Traits

```rust

```

## `macro_rules!`

```rust

```

## Error Handling

```rust

```

## Std Library Types

```rust

```

## Std mics

```rust

```

## Testing

```rust

```

## Unsafe Operations

```rust

```

## Compatibility

```rust

```

## Meta

```rust

```

## Reference

- [Simon Park ,"러스트의 멋짐을 모르는 당신은 불쌍해요"](https://parksb.github.io/article/35.html)
- [Tino Care, "How Microsoft Is Adopting Rust"](https://medium.com/@tinocaer/how-microsoft-is-adopting-rust-e0f8816566ba)

