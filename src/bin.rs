use brancher::git::{get_changes, is_default_branch};
use brancher::jira::JiraClient;
use brancher::ai;
use anyhow::Result;

fn main() -> Result<()>{
    env_logger::init();

    let jira = JiraClient::new()?;
    let issues = jira.get_user_issues()?;
    let diff = get_changes()?;

    let content = ai::choose_ticket(diff, issues)?;
    println!("{}", content);

    Ok(())
}
