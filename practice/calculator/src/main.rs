use std::io;

fn main() {
    let operator = "+";
    let x = prompt("Enter a value: ");
    let y = prompt("Enter a second value: ");

    // // convert values from String to i32
    // if operator == "+" || operator == "-" {
    //     let x: i32 = x.trim().parse().expect("Must be an integer");
    //     let y: i32 = y.trim().parse().expect("Must be an integer");
    // }


    let result = match operator {
        "+" => {
            let x: i32 = x.trim().parse().expect("Must be an integer");
            let y: i32 = y.trim().parse().expect("Must be an integer");
            addition(x, y).to_string()
        },
        "-" => {
            let x: i32 = x.trim().parse().expect("Must be an integer");
            let y: i32 = y.trim().parse().expect("Must be an integer");
            subtract(x, y).to_string()
        },
        // "*" => multiply(x, y).to_string(),
        // "/" => match divid(x, y) {
        //     Some(result) => result.to_string(),
        //     None => "Cannot divide by zero".to_string(),
        // },
        _ => "Invalid operator".to_string(),
    };

    println!("Result: {}", result);

    // println!("{}", addition(&[500, 100, 235, 1000, 9999]));
    // println!("{}", subtraction(4, 1));
}

fn addition(x: i32, y: i32) -> i32 {
    x + y
}

// fn addition_array(arr: &[i32]) -> i32 {
//     // return sum of Iterator (array)
//     arr.iter().sum()
// }

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

// fn subtraction(arr: &[i32]) -> i32 {
//     let mut result: i32 = 0;
//     for (i, x) in values.iter_mut().enumerate() [
//         if i == arr.len() { result - x }
//         *x -= arr[i + 1]
//         result = x
//     ]
// }

// fn multiply(x: i32, y: i32) -> i32 {
//     x * y
// }

// fn divid(x: i32, y: i32) -> Option<i32> {
//     if y != 0 {
//         Some(x / y)
//     } else {
//         None
//     }
// }

fn prompt(prompt_msg: &str) -> String {
    // ask for a number to use in equation
    print!("{}", prompt_msg);

    // trigger flush to display inline print()
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    // initialize string used as input value
    let mut input = String::new();
    
    // read input as string
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    // // set input string to variable and convert to u64
    // let input: u64 = input.trim().parse().expect("Must be an integer");

    // return input as u64
    input
}

// fn remainder(x:u64, y:u64) -> u64 {
//     x % y
// }
