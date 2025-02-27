mod fibonacci;
use std::env::args;

use fibonacci::fib;fn main() {
    let args: Vec<String> = args().skip(1).collect();

    if args.is_empty() {
        println!("No arguments supplied!");
        return;
    } else if args.len() != 2 {
        println!("FibBot requires exactly two parameters!");
        return;
    }

    let enable_fib = args[0].to_lowercase() == "true";
    
    let max_threshold= match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid argument. max_threshold must be an integer!");
            return;
        }
    };

    if enable_fib {
        println!("FibBot enabled successfully with max_threshold: {}", max_threshold);
        let fib_number = fib(max_threshold);
        println!("Fibonacci number: {}", fib_number);
    } else {
        println!("FibBot is disabled!");
    }
}