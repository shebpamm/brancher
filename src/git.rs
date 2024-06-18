use anyhow::Error;
use git2::Repository;

pub fn is_default_branch() -> Result<bool, Error> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?;
    let head_name = head.shorthand().unwrap();
    let default_branch = repo.config()?.get_string("init.defaultBranch")?;
    Ok(head_name == default_branch)
    
}
