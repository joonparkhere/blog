pub fn main() {
    types();
    literals_and_operators();
    tuples();
    arrays_and_slices();
}

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
