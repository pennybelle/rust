fn main() {
    // v0.1.0
    println!("Hello, world! This is a test. Now im testing the cargo run command >:3");

    // v0.1.1 learning about variables
    println!("---");

    let mut x = 4;
    println!("x = {}", x);
    x = 5;
    println!("x now = {}", x);

    // v0.1.2 more variables
    println!("---");

    let x = 4;
    println!("x = {}", x);
    let x = 5;
    println!("x now = {}", x);

    // v0.1.3 and more
    println!("---");

    let x = 4;
    println!("x = {}", x);
    let x = x + 1;
    println!("x now = {}", x);

    // v0.1.4 learning about scopes
    println!("---");

    let x = 4;
    println!("x = {}", x);
    {
        let x = 5;
        println!("x = {}", x);
    };
    {
        let x = "hi there";
        println!("x = {}", x);
    };
    println!("x = {}", x);

    // v0.1.5 difference between int and float
    println!("---");

    let x: u32 = 1000; // int (only positive)
    println!("x = {}", x);
    let x: i32 = 1000; // int (positive or negative)
    println!("x = {}", x);

    let x: f32 = 1000.0; // float (single precision)
    println!("x = {}", x);
    let x: f64 = 1000.0; // float (single precision)
    println!("x = {}", x);
}
