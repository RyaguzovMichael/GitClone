use std::io::{Write, stderr, stdout};
use std::process::Command;

pub fn get_project_name(git_url: &str) -> &str {
    let start_index_of_project_name = git_url.rfind('/').unwrap() + 1;
    let end_index_of_project_name = git_url.rfind('.').unwrap();
    &git_url[start_index_of_project_name..end_index_of_project_name]
}

pub fn clone(git_url: &str, destination_path: &str) {
    let child = Command::new("git")
        .arg("clone")
        .arg(git_url)
        .arg(destination_path)
        .spawn()
        .expect("Failed to execute command");

    let output = child.wait_with_output().unwrap();

    stdout().write_all(&output.stdout).unwrap();
    stderr().write_all(&output.stderr).unwrap();
}
