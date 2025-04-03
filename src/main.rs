mod git;
mod settings;

use std::env;

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

    git::run_git_clone_command(&git_url, &target_directory);
}

fn get_project_name(git_url: &str) -> &str {
    let start_index_of_project_name = git_url.rfind('/').unwrap() + 1;
    let end_index_of_project_name = git_url.rfind('.').unwrap();
    &git_url[start_index_of_project_name..end_index_of_project_name]
}
