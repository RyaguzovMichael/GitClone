use std::{env, fs};
use std::io::{Write, stdout, stderr};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let git_path = match args.get(1) {
        Some(x) => x,
        None => panic!("Input git path is empty"),
    };
    let work_directory = get_work_directory(&args[0]);
    let project_name = get_project_name(git_path).to_lowercase();
    let target = get_target(&work_directory);
    let destination_path = format!("{target}\\{project_name}");

    run_git_clone_command(&git_path, &destination_path);
}

fn get_work_directory(executable_path: &str) -> &str {
    let last_index = executable_path.rfind('\\').unwrap();
    &executable_path[..last_index]
}

fn get_project_name(git_path: &str) -> &str {
    let start_index_of_project_name = git_path.rfind('/').unwrap() + 1;
    let end_index_of_project_name = git_path.rfind('.').unwrap();
    &git_path[start_index_of_project_name..end_index_of_project_name]
}

fn get_target(work_directory: &str) -> String {
    let config_path = format!("{}\\target.config", work_directory);
    fs::read_to_string(config_path).unwrap()
}

fn run_git_clone_command(git_path: &str, destination_path: &str) {
    let output = Command::new("git")
        .arg("clone")
        .arg(git_path)
        .arg(destination_path)
        .output()
        .expect("Failed to execute command");

    stdout().write_all(&output.stdout).unwrap();
    stderr().write_all(&output.stderr).unwrap();
}