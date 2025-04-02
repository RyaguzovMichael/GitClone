mod settings;

use std::env;
use std::io::{stderr, stdout, Write};
use std::process::Command;

use settings::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let git_url = match args.get(1) {
        Some(x) => x,
        None => panic!("Input git url is empty"),
    };
    let project_name = get_project_name(git_url).to_lowercase();
    let settings = match Config::load() {
        Ok(config) => config,
        Err(error) => match Config::default() {
            Some(default) => default,
            None => panic!("{}", error.message),
        },
    };
    let target_directory = match settings
        .work_directory
        .join(project_name)
        .as_os_str()
        .to_str()
    {
        Some(res) => String::from(res),
        None => panic!("No work directory to push"),
    };

    run_git_clone_command(&git_url, &target_directory);
}

fn get_project_name(git_url: &str) -> &str {
    let start_index_of_project_name = git_url.rfind('/').unwrap() + 1;
    let end_index_of_project_name = git_url.rfind('.').unwrap();
    &git_url[start_index_of_project_name..end_index_of_project_name]
}

fn run_git_clone_command(git_url: &str, destination_path: &str) {
    let output = Command::new("git")
        .arg("clone")
        .arg(git_url)
        .arg(destination_path)
        .output()
        .expect("Failed to execute command");

    stdout().write_all(&output.stdout).unwrap();
    stderr().write_all(&output.stderr).unwrap();
}
