mod fibonacci;
mod fib_vector;
mod get_pr_content;
mod extract_numbers;

use extract_numbers::extract_numbers;
use fib_vector::fibonacci_vector;
use std::env::args;

use fibonacci::fib;
use get_pr_content::get_pull_request; fn main() {

    let content = get_pull_request();

    let content_numbers = extract_numbers(content);

    let args: Vec<String> = args().skip(1).collect();

    if args.is_empty() {
        println!("No arguments supplied!");
        return;
    } else if args.len() != 2 {
        println!("FibBot requires exactly two parameters!");
        return;
    }

    let enable_fib = args[0].to_lowercase() == "true";
    
    let max_threshold: i128 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid argument. max_threshold must be an integer!");
            return;
        }
    };

    if enable_fib {
        println!("FibBot enabled successfully with max_threshold: {}", max_threshold);
        let fib_numbers = fibonacci_vector(content_numbers);

        for fib_number in fib_numbers.iter() {
            if *fib_number <= max_threshold {
            println!("Fibonacci number: {}", fib_number);
            }
        }
        println!("Fibonacci numbers: {:?}", fib_numbers);
    } else {
        println!("FibBot is disabled!");
    }
}