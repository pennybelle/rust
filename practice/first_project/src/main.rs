fn v010() {
    // v0.1.0
    println!("--- 0.1.0 ---");

    println!("Hello, world! This is my first project.");
    println!("Each time I learn something i'll try it out here");
    println!("(at least till im good enough to not have to)");
}

fn v011() {
    // v0.1.1
    println!("--- 0.1.1 ---");

    let mut x = 4;
    println!("x = {}", x);
    x = 5;
    println!("x now = {}", x);
}

fn v012() {
    // v0.1.2 more variables
    println!("--- 0.1.2 ---");

    let x = 4;
    println!("x = {}", x);
    let x = 5;
    println!("x now = {}", x);
}

fn v013() {
    // v0.1.3 and more
    println!("--- 0.1.3 ---");

    let x = 4;
    println!("x = {}", x);
    let x = x + 1;
    println!("x now = {}", x);
}

fn v014() {
    // v0.1.4 learning about scopes
    println!("--- 0.1.4 ---");

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
    println!("--- 0.1.5 ---");

    let x: i32 = 1000; // int (signed integer) (positive or negative) (default)
    println!("x = {}", x);
    let x: u32 = 1000; // int (unsigned integer) (only positive)
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
    println!("--- 0.1.6 ---");

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
    println!("--- 0.1.7 ---");

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
    println!("--- 0.1.8 ---");

    // the number next to the letter represents
    // the number of bits used to store the int
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

    // another ways to declare type
    let x = 5i8;
    println!("x = {}", x);
    let x = 5i16;
    println!("x = {}", x);
    let x = 5_i32;
    println!("x = {}", x);
    let x = 5_i64;
    println!("x = {}", x);
    let x = 5 as i128;
    println!("x = {}", x);
}

fn v019() {
    // 0.1.9 floating point data type variations in depth
    println!("--- 0.1.9 ---");

    // the number next to the letter represents
    // the number of bits used to store the float
    let x: f32 = 5.5;
    println!("x = {}", x);
    let x: f64 = 5.5;
    println!("x = {}", x);

    // another ways to declare type
    let x = 5.5f32;
    println!("x = {}", x);
    let x = 5.5 as f64;
    println!("x = {}", x);
}

fn v0110() {
    // 0.1.10 tuples!!!
    println!("--- 0.1.10 ---");

    // how to index tuple and print tuple contents
    let test: (i32, bool, char) = (1, true, 's');
    println!("test = {}, {}, {}", test.0, test.1, test.2);

    // you can make tuples mutable.....wtf...
    let mut mutable_tuple: (i32, f64, i32) = (2, 5.5, 7);
    println!(
        "tuple before mutation = {}, {}, {}",
        mutable_tuple.0, mutable_tuple.1, mutable_tuple.2
    );
    mutable_tuple.0 = 1;
    mutable_tuple.1 = 2.5;
    mutable_tuple.2 = 5;
    println!(
        "tuple after mutation = {}, {}, {}",
        mutable_tuple.0, mutable_tuple.1, mutable_tuple.2
    );

    // overwriting a tuple
    let tup: (bool, char, f64) = (true, 's', 5.5);
    println!("tuple before overwrite = {}, {}, {}", tup.0, tup.1, tup.2);
    let tup: (i32, i32, bool) = (1, 2, false);
    println!("tuple after overwrite = {}, {}, {}", tup.0, tup.1, tup.2);
}

fn v0111() {
    // 0.1.11 arrays!!!
    println!("--- 0.1.11 ---");

    // creating and indexing an array
    let arr = [1, 2, 3]; // all values must be same type
    println!("array = {}, {}, {}", arr[0], arr[1], arr[2]);

    // creating and changing a mutable array
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 6;
    println!("{}", arr[4]);

    // reassign arr to mutable array with length of 3 values instead of 5
    let arr: [i32; 3] = [1, 5, 10];
    println!("{}, {}, {}", arr[0], arr[1], arr[2]);
}

fn v0112() {
    println!("--- 0.1.12 crates and inputs!!! ---");

    // import io (input/output) module from std (standard library) crate
    use std::io;

    // create mutable string to assign the input
    let mut input = String::new();

    // on input, modify String (input)
    // & is a reference, immutable by default so i make it mutable
    // read_line returns a Result Object
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    // // this would send a copy of input instead of the original
    // io::stdin().read_line(input);

    // send reply after input is entered
    println!("input: {}", input);
}

fn v0113() {
    println!("--- 0.1.13 input testing ---");

    use std::io;

    let mut input = String::new();

    println!("right or left?");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("you chose {}", input);
}

fn v0114() {
    println!("--- 0.1.14 changing data types ---");

    println!("f32 max = {}", f32::MAX);
    let x = (f32::MAX as f64) + 1.0;
    let y: f32 = 10.0;

    let z = x + (y as f64);
    println!("{}", z);

    // let x = 5 as i8;
    // let y = 10.5 as f64;

    // let z = (x as f64) + y;
    // println!("{}", z)
}

fn prank() {
    use std::io;

    // UNFINISHED
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("couldnt read line...");
        println!(" {}", input);
    }
}

fn v0115() {
    println!("--- 0.1.15 my first rust calculator ---");

    // import container
    use std::io;

    // initialize strings
    let mut input = String::new();
    let mut input2 = String::new();

    // initialize input and connect to string
    io::stdin()
        .read_line(&mut input)
        .expect("couldn't read line...");
    io::stdin()
        .read_line(&mut input2)
        .expect("couldn't read line...");

    // string to integer conversion
    let int_input: f64 = input.trim().parse().unwrap();
    let int_input2: f64 = input2.trim().parse().unwrap();

    // ADD 'EM UP
    let result = int_input + int_input2;

    println!("{}", result);
}

fn v0116() {
    println!("--- 0.1.16 conditions ---");

    // logical operators:
    // && is and
    // || is or
    // ! is not

    // initialize variable and store result of comparison
    let cond = (1 as f32) <= 2.2;
    println!("{}", cond);
    let cond2 = (2.2 as f32) <= 2.2;
    println!("{}", cond2);

    // return true only if both conditions are true (and gate)
    let result = cond && cond2;
    println!("{}", result);

    // return true if either or both conditions are true (or gate)
    let result2 = cond || cond2;
    println!("{}", result2);
}

fn v0117() {
    println!("--- 0.1.17 format specifiers ---");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    let num = 10 as i32;
    for i in 0..num {
        println!("---");
        println!("Base 10:               {}", i); // 69420
        println!("Base 2 (binary):       {:b}", i); // 10000111100101100
        println!("Base 8 (octal):        {:o}", i); // 207454
        println!("Base 16 (hexadecimal): {:x}", i); // 10f2c
    }

    println!("{i:>5}", i = 1);
    println!("{i:0<6}", i = 1);
}

fn v0118() {
    println!("--- 0.1.18 creating arrays from a range ---");

    // create an array with 100 elements in it (using turbofish type-casting)
    let array = (1..=100).collect::<Vec<u32>>();

    println!("{}", array.len());
}

fn string_example() {
    // implicit type
    let string_ex = "string";
    println!("{}", string_ex);

    // explicit type
    let string_ex = String::from("string 2");
    println!("{}", string_ex);
}

fn main() {
    // v010();
    // v011();
    // v012();
    // v013();
    // v014(); // scopes
    // v015();
    // v016();
    // v017(); // data types (int, float, bool, char)
    // v018(); // ints in depth
    // v019(); // floats in depth
    // v0110(); // tuples
    // v0111(); // arrays
    // v0112(); // crates and inputs
    // v0113(); // input testing
    // v0114(); // changing data types
    // v0115(); // ADD 'EM UP (basic calculator)
    // v0116(); // conditions
    // v0117();
    // v0118();
    string_example();
}
