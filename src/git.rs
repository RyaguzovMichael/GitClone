use std::io::{self, Write, stderr, stdout};
use std::process::Command;

pub fn get_project_name(git_url: &str) -> Option<&str> {
    let start_index_of_project_name = git_url.rfind('/')? + 1;
    let end_index_of_project_name = git_url.rfind('.')?;
    Some(&git_url[start_index_of_project_name..end_index_of_project_name])
}

pub fn clone(git_url: &str, destination_path: &str) -> Result<(), io::Error> {
    let output = Command::new("git")
        .arg("clone")
        .arg(git_url)
        .arg(destination_path)
        .spawn()?
        .wait_with_output()?;

    stdout().write_all(&output.stdout)?;
    stderr().write_all(&output.stderr)?;
    return Ok(());
}
