use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut number_of_guesses = 0;

    println!("Guess the number!");

    // generate random number between 0 and 100
    let number: i64 = rand::thread_rng().gen_range(1, 101);

    // loop until number is guessed correctly
    loop {
        number_of_guesses += 1;

        // assign input to string
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Line could not be read");

        // convert input from string to integer (String to u32)
        let guess: i64 = guess.trim().parse().expect("Please type a number!");

        // compare guess and randomly generated number
        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Took {} guesses.", number_of_guesses);
                // std::process::exit(0);
                break;
            }
        }
    }
}
