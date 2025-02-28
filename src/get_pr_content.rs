// use std::{env, process::Command};

// pub fn get_pull_request() -> String {

//     let pr_number = env::var("PR_NUMBER")
//         .expect("PR_NUMBER not set")
//         .parse::<u32>()
//         .expect("Invalid PR_NUMBER");

//     let repo = env::var("GITHUB_REPOSITORY");
//     let token = env::var("GITHUB_TOKEN");

//     let output = Command::new("curl")
//         .arg("-L")
//         .arg("-H")
//         .arg("Accept: application/vnd.github+json")
//         .arg("-H")
//         .arg("Authorization: token")
//         .arg("-H")
//         .arg("X-GitHub-Api-Version: 2022-11-28")
//         .arg("https://api.github.com/repo/pulls/pr_number/files")
//         .output()
//         .expect("Failed to get pull request content");

//     String::from_utf8_lossy(&output.stdout).to_string()
// }

// use reqwest::blocking::Client;
// use std::env;

// pub fn get_pr_body(pr_number: u32) -> Result<String, Box<dyn std::error::Error>> {

//     let repo = env::var("GITHUB_REPOSITORY")?;
//     let token = env::var("GITHUB_TOKEN")?;
//     let url = format!("https://api.github.com/repos/{}/pulls/{}/files", repo, pr_number);

//     let client = Client::new();
//     let response = client
//         .get(&url)
//         .header("User-Agent", "FibBot")
//         .header("Accept", "application/vnd.github.full+json")
//         .bearer_auth(token)
//         .send()?;

//     if response.status().is_success() {
//         let json: serde_json::Value = response.json()?;
//         if let Some(body) = json.get("body") {
//             return Ok(body.as_str().unwrap_or("").to_string());
//         }
//     }

//     Err("Failed to get pull_request body".into())
// }

use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct PullRequest {
    title: String,
    body: Option<String>,
    html_url: String,
}

pub async fn get_pull_request(
    owner: &str,
    repo: &str,
    pr_number: u64,
    github_token: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    // GitHub API endpoint for a specific pull request
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        owner, repo, pr_number
    );

    // Send a GET request to the GitHub API
    let response = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .header("Authorization", format!("Bearer {}", github_token))
        .send()
        .await?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch pull request: {}",
            response.status()
        )
        .into());
    }

    // Parse the JSON response into a PullRequest struct
    let pr: PullRequest = response.json().await?;

    // Format the pull request content as a string
    let pr_content = format!(
        "Title: {}\nBody: {}\nURL: {}",
        pr.title,
        pr.body.unwrap_or_else(|| "No description provided.".to_string()),
        pr.html_url
    );

    Ok(pr_content)
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let owner = "rust-lang"; // Repository owner
//     let repo = "rust";       // Repository name
//     let pr_number = 12345;   // Pull request number
//     let github_token = "your_github_personal_access_token"; // Replace with your GitHub token

//     match get_pull_request(owner, repo, pr_number, github_token).await {
//         Ok(content) => println!("Pull Request Content:\n{}", content),
//         Err(e) => eprintln!("Error: {}", e),
//     }

//     Ok(())
// }