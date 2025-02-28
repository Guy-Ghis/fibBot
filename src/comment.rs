use reqwest::blocking::Client;
use std::env;

pub fn post_comment(pr_number: u32, owner: &str, repo: &str, comment: String) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN")?;
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}/comments", owner, repo, pr_number);

    let client = Client::new();
    let response = client
        .post(&url)
        .header("User-Agent", "rust-github-action")
        .bearer_auth(token)
        .json(&serde_json::json!({ "body": comment }))
        .send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to post comment: {} - {}", response.status(), comment).into())
    }
}
use reqwest::blocking::Client;
use std::env;

pub fn post_comment(pr_number: u32, owner: &str, repo: &str, comment: String) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN")?;
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}/comments", owner, repo, pr_number);

    let client = Client::new();
    let response = client
        .post(&url)
        .header("User-Agent", "rust-github-action")
        .bearer_auth(token)
        .json(&serde_json::json!({ "body": comment }))
        .send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Failed to post comment: {} - {}", response.status(), comment).into())
    }
}