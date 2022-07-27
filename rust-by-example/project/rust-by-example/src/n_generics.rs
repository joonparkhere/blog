pub fn main() {
    generics_functions();
    generics_implementations();
    generics_traits();
    generics_bounds();
    generics_multiple_bounds();
    generics_where_clauses();
    generics_new_type_idiom();
    generics_associated_items();
    generics_phantom_type_params();
}

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
