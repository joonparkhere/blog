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
