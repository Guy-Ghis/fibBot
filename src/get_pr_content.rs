use crate::extract_numbers::extract_numbers;
use octocrab::models::pulls::PullRequest;
use octocrab::Octocrab;

pub async fn get_pull_request(pr_number: u64) -> Vec<i128>{

    let files = octocrab::instance().pulls("Guy-Ghis", "fibBot").list_files(4).await;
    let files = files.unwrap().items.first().unwrap().patch.clone().unwrap();
    println!("Pull Resquest Contents:\n{}",files);
    let collected_numbers = extract_numbers(&files.as_str().to_string());
    println!("Collected Nums: {:?}", collected_numbers);
    collected_numbers
 }