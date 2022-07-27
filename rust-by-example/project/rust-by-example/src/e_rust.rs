#![allow(overflowing_literals)]

pub fn main() {
    casting();
    literals();
    inference();
    aliasing();
}

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
