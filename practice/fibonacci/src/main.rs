fn main() {
    fibonacci(25);
}

fn fibonacci(depth: u64) {
    let mut last: u64 = 0;
    let mut current: u64 = 1;

    for _ in 0..depth {
        println!("{}", last);

        let cache: u64 = current + last;
        last = current;
        current = cache;
    }
}
