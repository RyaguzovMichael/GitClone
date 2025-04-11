mod error;
mod git;
mod settings;

use error::Error;
use settings::Config;

const INVALID_PROJECT_URL: &str = "Can't be get project name from git url";
const INVALID_CONFIG_DIRECTORY: &str = "No work directory to push";

pub fn clone(git_url: &str) -> Result<(), Error> {
    let project_name = match git::get_project_name(git_url) {
        Some(name) => name.to_lowercase(),
        None => return Err(Error::from(INVALID_PROJECT_URL)),
    };
    let settings = Config::load()?;

    let target_directory = match settings
        .work_directory
        .join(project_name)
        .as_os_str()
        .to_str()
    {
        Some(res) => String::from(res),
        None => return Err(Error::from(INVALID_CONFIG_DIRECTORY)),
    };

    git::clone(&git_url, &target_directory)?;
    Ok(())
}
