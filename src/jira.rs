use gouqi::{Credentials, Jira};
use anyhow::{Context, Result};
use std::env;

pub struct JiraClient {
    client: Jira,
}

pub struct Issue {
    pub key: String,
    pub summary: String,
    pub description: String,
}

pub type Issues = Vec<Issue>;

impl JiraClient {
    pub fn new() -> Result<Self> {
        let host = env::var("JIRA_HOST").context("JIRA_HOST not defined")?;
        let username = env::var("JIRA_USERNAME").context("JIRA_USERNAME not defined")?;
        let jira_token = env::var("JIRA_API_TOKEN").context("JIRA_API_TOKEN not defined")?;

        let credentials = Credentials::Basic(username, jira_token);
        let client = Jira::new(host, credentials)?;
        Ok(Self { client })
    }

    pub fn get_user_issues(&self) -> Result<Issues> {
        match self.client.search().iter("project = DEVOPS AND status in (\"In Progress\", Review, \"To Do\") AND assignee in (currentUser()) order by created DESC", &Default::default()) {
            Ok(results) => {
                let mut issues = Vec::new();
                for issue in results {
                    issues.push(Issue {
                        key: issue.key.to_string(),
                        summary: issue.summary().expect("No summary"),
                        description: issue.description().unwrap_or("".to_string())
                    });
                }

                return Ok(issues);
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        };
        Ok(Vec::new())
    }
}
