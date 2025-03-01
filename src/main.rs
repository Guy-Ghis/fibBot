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

        let pr_number = args.get(3).unwrap_or(&"1".to_string()).parse::<u64>().unwrap_or(1);

    println!("FibBot application is running...");
    println!("Fibonacci Calculation Enabled: {}", enable_fib);
    println!("Max Threshold is: {}", max_threshold);
    println!("the pull_request number is: {}",pr_number);


let github_repository =
        env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "Guy-Ghis/fibBot".to_string());
    let github_repository_vec = github_repository.split("/").collect::<Vec<&str>>();
    let owner = github_repository_vec[0];
    let repo = github_repository_vec[1];

let pr_files = get_pull_request(owner, repo, pr_number).await;
println!("Extracted numbers: {:?}", pr_files);

let pr_files = match pr_files {
    Ok(files) => files,
    Err(e) => {
        eprintln!("Error fetching pull request files: {}", e);
        return;
    }
};

if pr_files.items.is_empty() {
    println!("No files found in this pull request.");
    return;
}

let path = &pr_files.items.first().unwrap().patch.clone().unwrap();

let pull_request_numbers = extract_numbers(path);
println!("Extracted numbers: {:?}", pull_request_numbers);

if pr_files.items.is_empty() {
    println!("No numbers found in this pull_request.");
}

let pr_content = pr_files.items.first().unwrap().patch.clone().unwrap();
let _ = post_comment(&pr_content).await;

let mut responses = Vec::new(); // Vector to store each Fibonacci result

for file in &pr_files {
    if let Some(num_str) = file.patch.as_ref().and_then(|patch| extract_numbers(patch).first().cloned()) {
        let num = num_str;
        let fib = fib(num.into());
        responses.push(format!("- Fibonacci({}) = {}", num, fib)); // Store each result
    }
}

// Construct the final response string
let mut response = String::from("Fibonacci output of each number in the pull_request is:\n");
for res in responses {
    response.push_str(&res);
    response.push('\n');
}

// Post the final response
if let Err(e) = post_comment(&response).await {
    eprintln!("Error posting comment: {}", e);
}

}