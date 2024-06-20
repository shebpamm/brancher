use brancher::git::{get_changes, is_default_branch};
use brancher::jira::JiraClient;
use anyhow::Result;

fn main() -> Result<()>{
    let jira = JiraClient::new()?;
    let issues = jira.get_user_issues()?;
    let diff = get_changes()?;

    println!("Diff: {}", diff);

    Ok(())
}
