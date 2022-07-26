pub fn main() {
    intro();
    associated_functions_and_methods();
    closures();
    closures_capturing();
    closures_move();
    closures_as_input_params();
    closures_type_anonymity();
    closures_input_functions();
    closures_as_output_params();
    closures_iterator_any();
    higher_order_functions();
    diverging_functions();
}

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
