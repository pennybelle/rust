// import container for input
use std::io;

fn main() {
    equation(prompt());
}

fn prompt() -> u64 {
    // ask for a number to use in equation
    print!("Enter a positive integer: ");

    // trigger flush to display inline print()
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    // initialize string used as input value
    let mut input = String::new();
    
    // read input as string
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    // set input string to variable and convert to u64
    let input: u64 = input.trim().parse().expect("Must be an integer");

    // return input as u64
    input
}

fn equation(input: u64) {
    let mut x = input;

    // loop over equation until x == 1
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2
        } else {
            x = 3 * x + 1
        }
        println!("{}", x);
    }
}
