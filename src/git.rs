use anyhow::Result;
use git2::Diff;
use git2::Repository;

pub fn is_default_branch() -> Result<bool> {
    let repo = Repository::open_from_env()?;
    let head = repo.head()?;
    let head_name = head.shorthand().unwrap();
    let default_branch = repo.config()?.get_string("init.defaultBranch")?;
    Ok(head_name == default_branch)
}

// Because I'm lazy this just runs the git command
pub fn get_changes() -> Result<String> {
    std::process::Command::new("git")
        .arg("diff")
        .output()
        .map(|output| String::from_utf8(output.stdout).unwrap())
        .map_err(|e| e.into())
}
