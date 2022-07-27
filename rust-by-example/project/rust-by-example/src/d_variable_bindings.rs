pub fn main() {
    intro();
    mutability();
    scope();
    shadowing();
    declare_first();
    freezing();
}

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
