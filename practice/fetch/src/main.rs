// use std::env;
use std::fs;

fn main() {
    let file_path = "/etc/os-release";
    // println!("{file_path}");

    let contents = fs::read_to_string(file_path)
    .unwrap();
    // println!("{contents}");

    let parse: String = contents
    .replace("\"", "")
    .split("=")
    .collect();
    println!("{parse}");

    let lines: String = parse.lines().collect();
    println!("{lines}");
}