use std::{env, process::Command};

pub fn get_pull_request() -> String {

    let pr_number = env::var("PR_NUMBER")
        .expect("PR_NUMBER not set")
        .parse::<u32>()
        .expect("Invalid PR_NUMBER");

    let repo = env::var("GITHUB_REPOSITORY");
    let token = env::var("GITHUB_TOKEN");

    let output = Command::new("curl")
        .arg("-L")
        .arg("-H")
        .arg("Accept: application/vnd.github+json")
        .arg("-H")
        .arg("Authorization: token")
        .arg("-H")
        .arg("X-GitHub-Api-Version: 2022-11-28")
        .arg("https://api.github.com/repo/pulls/pr_number/files")
        .output()
        .expect("Failed to get pull request content");

    String::from_utf8_lossy(&output.stdout).to_string()
}