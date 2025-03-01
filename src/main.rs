mod fibonacci;
mod get_pr_content;
mod extract_numbers;

use fibonacci::fib;
use extract_numbers::extract_numbers;
use std::env::{self, args};
use get_pr_content::get_pr_body;
use get_pr_content::get_pull_request;

fn main() {

    let args: Vec<String> = env::args().collect();

    let pr_number = &args[1];
    let pr_body = &args[2];
    let github_token = &args[3];
    
    // let pr_number = octocrab
    // .pulls(&owner, &repo)
    // .get(pr_number)
    // .await
    // .expect("Failed to get pull request")?;

    // let owner = "Guy-Ghis";
    // let repo = env::var("GITHUB_REPOSITORY")?;
    // let github_token = env::var("GITHUB_TOKEN")?;

// let content = get_pull_request(pr_number, &owner, &repo, &github_token)
//     .expect("Failed to get pull request content");

    let content_numbers: Vec<i128> = pr_body
        .split_whitespace()
        .filter_map(|word| word.parse()::<i128>().ok())
        .collect();

    let results = String::new();

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

        for number in numbers {
            let fib_answers = fib(number);
            results.push_str(&format!("Fibonacci({}) = {}\n", number, fib_answers));
        }

        // for number in content_numbers.iter() {
        //     if *number <= max_threshold {
        //         let fib_number = fib(*number);
        //         println!("Fibonacci numbers: {:?}", fib_number);
        //     }
        // }
    } else {
        println!("FibBot is disabled!");
    }
}