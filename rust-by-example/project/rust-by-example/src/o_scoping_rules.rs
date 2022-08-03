pub fn main() {
    raii();
    raii_destructor();
    ownership_and_moves();
    ownership_and_movers_of_mutability();
    partial_moves();
    borrowing();
    borrowing_mutability();
    borrowing_aliasing();
    borrowing_the_ref_pattern();
    lifetime();
    lifetime_explicit_annotation();
    lifetime_functions();
    lifetime_methods();
    lifetime_structs();
    lifetime_traits();
    lifetime_bounds();
    lifetime_coercion();
    lifetime_static();
    lifetime_elision();
}

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
