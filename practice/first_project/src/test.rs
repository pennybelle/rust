use std::io;

fn input_test() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failt to read line")

    println!("{}", input)
}