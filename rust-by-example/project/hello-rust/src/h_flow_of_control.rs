pub fn main() {
    if_else();
    loop_and_break();
    nesting_and_labels();
    returning_from_loops();
    while_loops();
    for_loops();
    match_keyword();
    match_destructuring();
    match_guards();
    match_binding();
    if_let();
    while_let();
}

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
