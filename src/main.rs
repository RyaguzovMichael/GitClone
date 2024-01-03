use std::{env, fs};
use std::io::{Write, stdout, stderr};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let git_url = match args.get(1) {
        Some(x) => x,
        None => panic!("Input git url is empty"),
    };
    let project_name = get_project_name(git_url).to_lowercase();
    let default_destination_path = get_default_destination_path(&args[0]);
    let destination_path = format!("{default_destination_path}\\{project_name}");

    run_git_clone_command(&git_url, &destination_path);
}

fn get_project_name(git_url: &str) -> &str {
    let start_index_of_project_name = git_url.rfind('/').unwrap() + 1;
    let end_index_of_project_name = git_url.rfind('.').unwrap();
    &git_url[start_index_of_project_name..end_index_of_project_name]
}

fn get_default_destination_path(executable_file_path: &str) -> String {
    let executable_directory = get_executable_directory(executable_file_path);
    let config_path = format!("{executable_directory}\\target.config");
    fs::read_to_string(config_path).unwrap()
}

fn get_executable_directory(executable_file_path: &str) -> &str {
    let last_index = executable_file_path.rfind('\\').unwrap();
    &executable_file_path[..last_index]
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