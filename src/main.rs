use std::{env, fs};
use std::io::{Write, stdout, stderr};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let git_path = args.get(1).unwrap();
    let work_directory = get_work_directory(&args);
    let project_name = get_project_name(git_path);
    let target = get_target(&work_directory);
    let destination_path = format!("{}\\{}", target, project_name);

    let output = Command::new("git")
        .arg("clone")
        .arg(git_path)
        .arg(&destination_path)
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    stdout().write_all(&output.stdout).unwrap();
    stderr().write_all(&output.stderr).unwrap();
}

fn get_work_directory(args: &Vec<String>) -> String {
    let executable_path = args.get(0).unwrap();
    let last_index = executable_path.rfind('\\').unwrap();
    executable_path[..last_index].to_string()
}

fn get_project_name(git_path: &String) -> String {
    if !git_path.ends_with(".git") {
        panic!("Unknown path to git")
    }

    let start_index_of_project_name = git_path.rfind('/').unwrap() + 1;
    let end_index_of_project_name = git_path.rfind('.').unwrap();
    git_path[start_index_of_project_name..end_index_of_project_name].to_lowercase().clone()
}

fn get_target(work_directory: &String) -> String {
    let config_path = format!("{}\\target.config", work_directory);
    fs::read_to_string(config_path).unwrap()
}