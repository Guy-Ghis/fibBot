mod fibonacci;
mod get_pr_content;
mod extract_numbers;

use fibonacci::fib;
use extract_numbers::extract_numbers;
use std::env::{self, args};
use get_pr_content::get_pr_body;

fn main() {

    let args: Vec<String> = args().skip(1).collect();
    
    let pr_number = env::var("PR_NUMBER")
    .expect("PR_NUMBER not set")
    .parse::<u32>()
    .expect("Invalid PR_NUMBER");

let content = get_pr_body(pr_number).unwrap();

let content_numbers = extract_numbers(content);

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

        for number in content_numbers.iter() {
            if *number <= max_threshold {
                let fib_number = fib(*number);
                println!("Fibonacci numbers: {:?}", fib_number);
            }
        }
    } else {
        println!("FibBot is disabled!");
    }
}