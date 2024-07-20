fn main() {
    fibonacci(100);
}

fn fibonacci(depth: i128) {
    let mut last: i128 = 0;
    let mut current: i128 = 1;
    let mut cache: i128 = 0;

    for _ in 0..=depth {
        println!("{}", last);
        cache = current + last;
        last = current;
        current = cache;
    }
}
