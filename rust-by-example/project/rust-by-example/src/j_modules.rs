pub fn main() {
    visibility();
    struct_visibility();
    use_declaration();
    super_and_self();
}

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
