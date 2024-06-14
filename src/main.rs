use tokio;
use git2::Repository;
use anyhow::Error;

fn is_default_branch() -> Result<bool, Error> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?;
    let head_name = head.shorthand().unwrap();
    let default_branch = repo.config()?.get_string("init.defaultBranch")?;
    Ok(head_name == default_branch)
    
}

#[tokio::main]
async fn main() {
    match is_default_branch() {
        Ok(true) => println!("On default branch"),
        Ok(false) => println!("Not on default branch"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
