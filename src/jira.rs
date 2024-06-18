use gouqi::{Credentials, Jira};
use anyhow::Error;
use std::env;

pub struct JiraClient {
    client: Jira,
}

impl JiraClient {
    pub fn new() -> Result<Self, Error> {
        let host = env::var("JIRA_HOST")?;
        let username = env::var("JIRA_USERNAME")?;
        let jira_token = env::var("JIRA_API_TOKEN")?;

        let credentials = Credentials::Basic(username, jira_token);
        let client = Jira::new(host, credentials)?;
        Ok(Self { client })
    }

    pub fn asd(&self) -> Result<(), Error> {
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
