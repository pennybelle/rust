fn v010() {
    // v0.1.0
    println!("Hello, world! This is my first project.");
    println!("Each time I learn something i'll try it out here");
    println!("(at least till im good enough to not have to)");
}

fn v011() {
    // v0.1.1
    println!("---");

    let mut x = 4;
    println!("x = {}", x);
    x = 5;
    println!("x now = {}", x);
}

fn v012() {
    // v0.1.2 more variables
    println!("---");

    let x = 4;
    println!("x = {}", x);
    let x = 5;
    println!("x now = {}", x);
}

fn v013() {
    // v0.1.3 and more
    println!("---");

    let x = 4;
    println!("x = {}", x);
    let x = x + 1;
    println!("x now = {}", x);
}

fn v014() {
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
}

fn v015() {
    // v0.1.5 difference between int and float
    println!("---");

    let x: i32 = 1000; // int (positive or negative) (default)
    println!("x = {}", x);
    let x: u32 = 1000; // int (only positive)
    println!("x = {}", x);

    let x: f64 = 1000.0; // float (double precision) (default)
    println!("x = {}", x);
    let x: f32 = 1000.0; // float (single precision)
    println!("x = {}", x);

    // messing with floats a bit more
    {
        let floating_point: f64 = 10.9;
        println!("floating_point = {}", floating_point)
    }
}

fn v016() {
    // v0.1.6 boolean type variants
    println!("---");

    // true
    let true_or_false: bool = true;
    println!("true_or_false = {}", true_or_false);
    // let true_or_false: bool = 1;
    // println!("true_or_false = {}", true_or_false);

    // false
    let true_or_false: bool = false;
    println!("true_or_false = {}", true_or_false);
    // let true_or_false: bool = 0;
    // println!("true_or_false = {}", true_or_false);
}

fn v017() {
    // v0.1.7 the main 4 different data types in rust
    println!("---");

    // integer (i32 by default)
    let x: i32 = 5;
    println!("x = {}", x);

    // float (f64 by default)
    let x: f64 = 5.5;
    println!("x = {}", x);

    // boolean (true or false)
    let x: bool = true;
    println!("x = {}", x);

    // character (single)
    let x: char = 'a';
    println!("x = {}", x);
}

fn v018() {
    // 0.1.8 integer data type variations in depth
    println!("---");

    let x: i8 = 5;
    println!("x = {}", x);
    let x: i16 = 5;
    println!("x = {}", x);
    let x: i32 = 5;
    println!("x = {}", x);
    let x: i64 = 5;
    println!("x = {}", x);
    let x: i128 = 5;
    println!("x = {}", x);
}

fn main() {
    v010();
    v011();
    v012();
    v013();
    v014(); // scopes
    v015();
    v016();
    v017(); // data types
    v018(); // ints in depth
}
