fn main() {
    // v0.1.0
    println!("Hello, world! This is a test. Now im testing the cargo run command >:3");

    // v0.1.1
    println!("---");
    
    let mut x = 4;
    println!("x = {}", x);
    x = 5;
    println!("x now = {}", x);

    // v0.1.2
    println!("---");

    let x = 4;
    println!("x = {}", x);
    let x = 5;
    println!("x now = {}", x);

    // v0.1.3
    println!("---");

    let x = 4;
    println!("x = {}", x);
    let x = x + 1;
    println!("x now = {}", x);

    // v0.1.4
    println!("---");

    let x = 4;
    println!("x = {}", x);
    {
        let x = 5;
        println!("x = {}", x);
    };
    println!("x = {}", x);
}
