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
    let project_name = git::get_project_name(git_url).to_lowercase();
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

    git::clone(&git_url, &target_directory);
}
