mod fibonacci;
mod get_pr_content;
mod extract_numbers;

use fibonacci::fib;
use extract_numbers::extract_numbers;
use std::env::{self, args};
use get_pr_content::get_pr_body;
use get_pr_content::get_pull_request;

async fn main() {

    let args: Vec<String> = args().skip(1).collect();
    
    let pr_number = octocrab
    .pulls(&owner, &repo)
    .get(pr_number)
    .await
    .expect("Failed to get pull request")?;

    let owner = "Guy-Ghis"; // Repository owner
    let repo = env::var("GITHUB_REPOSITORY")?;
    let github_token = env::var("GITHUB_TOKEN")?;

let content = get_pull_request(pr_number, &owner, &repo, &github_token)
    .expect("Failed to get pull request content");

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