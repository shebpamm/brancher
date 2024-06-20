use gouqi::{Credentials, Jira};
use anyhow::{Context, Result};
use std::env;

pub struct JiraClient {
    client: Jira,
}

impl JiraClient {
    pub fn new() -> Result<Self> {
        let host = env::var("JIRA_HOST").context("JIRA_HOST not defined")?;
        let username = env::var("JIRA_USERNAME").context("JIRA_USERNAME not defined")?;
        let jira_token = env::var("JIRA_API_TOKEN").context("JIRA_API_TOKEN not defined")?;

        let credentials = Credentials::Basic(username, jira_token);
        let client = Jira::new(host, credentials)?;
        Ok(Self { client })
    }

    pub fn asd(&self) -> Result<()> {
        match self.client.search().iter("created >= -30d AND assignee = currentUser() ORDER BY created DESC", &Default::default()) {
            Ok(results) => {
                for issue in results {
                    println!("{:?}:", issue.assignee());
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        };
        Ok(())
    }
}
