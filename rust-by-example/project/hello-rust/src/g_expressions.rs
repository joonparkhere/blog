pub fn main() {
    expressions();
}

fn expressions() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
        // This no semicolon expression will be assigned to `y`
    };

    let z = {
        2 * x;
        // The semicolon suppresses this expression and `()` is assigned to `z`
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    // x is 5
    // y is 155
    // z is ()
}
