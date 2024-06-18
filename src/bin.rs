use brancher::git::is_default_branch;
use brancher::jira::JiraClient;

fn main() {
    let jira = JiraClient::new().unwrap();
    jira.asd().unwrap();
}
