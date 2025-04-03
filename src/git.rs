use std::io::{Write, stderr, stdout};
use std::process::Command;

pub fn run_git_clone_command(git_url: &str, destination_path: &str) {
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
