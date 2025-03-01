use octocrab::{models::repos::DiffEntry, Page};

pub async fn get_pull_request(owner: &str, repo: &str, pr_number: u64) -> Result<Page<DiffEntry>, octocrab::Error> {
    octocrab::instance().pulls(owner, repo).list_files(pr_number).await
}