mod fibonacci;
mod get_pr_content;
mod extract_numbers;
mod comment;

use fibonacci::fib;
use comment::post_comment;
use extract_numbers::extract_numbers;
use get_pr_content::get_pull_request;
use std::env::{self};
use tokio;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    let enable_fib = args.get(1).unwrap_or(&"true".to_string()).to_lowercase() == "true";
    let max_threshold: u8 = args
        .get(2)
        .unwrap_or(&"100".to_string())
        .parse()
        .unwrap_or(100);

    // let pr_body = &args[2];
    // let github_token = &args[3];
    
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

println!("FibBot application is running...");
println!("Fibonacci Calculation Enabled: {}", enable_fib);
println!("Max Threshold is: {}", max_threshold);

let pr_number: u64 = env::var("INPUT_PR_NUMBER")
    .expect("PR_NUMBER not set")
    .parse::<u64>()
    .expect("Invalid PR_NUMBER");

println!("the pull_request number is: {}",pr_number);

let pr_numbers = get_pull_request(pr_number).await;
println!("Extracted numbers: {:?}", pr_numbers);

if pr_numbers.is_empty() {
    println!("No numbers found in this pull_request.");
}
let mut response =
    String::from("#### Fibonacci output of each number in the pull_request is:\n");
for &num in &pr_numbers {
    let fib = fib(num);
    response.push_str(&format!("- Fibonacci({}) = {}\n", num, fib));
}
    if let Err(e) = post_comment(&response).await {
        eprintln!("Error posting comment: {}", e);
    }
}

        // for number in content_numbers.iter() {
        //     if *number <= max_threshold {
        //         let fib_number = fib(*number);
        //         println!("Fibonacci numbers: {:?}", fib_number);
        //     }
        // }
//     } else {
//         println!("FibBot is disabled!");
//     }
// }