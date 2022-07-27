pub fn main() {
    intro();
    dead_code();
    cfg();
}

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
