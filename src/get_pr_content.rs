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

use reqwest::blocking::Client;
use std::env;

pub fn get_pr_body(pr_number: u32) -> Result<String, Box<dyn std::error::Error>> {

    let repo = env::var("GITHUB_REPOSITORY")?;
    let token = env::var("GITHUB_TOKEN")?;
    let url = format!("https://api.github.com/repos/{}/pulls/{}/files", repo, pr_number);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .bearer_auth(token)
        .send()?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json()?;
        if let Some(body) = json.get("body") {
            return Ok(body.as_str().unwrap_or("").to_string());
        }
    }

    Err("Failed to get pull_request body".into())
}