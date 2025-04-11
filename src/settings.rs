use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

use crate::Error;

const CONFIG_FILE_NAME: &str = "config";

pub(crate) struct Config {
    pub work_directory: PathBuf,
}

impl Config {
    pub(crate) fn load() -> Result<Config, Error> {
        match dirs::home_dir() {
            Some(home_dir) => {
                let config_path = home_dir.join(".config/gclone");
                let config = match get_config_file_data(&config_path) {
                    Ok(file_data) => Config::deserialize(file_data.trim())?,
                    Err(_) => {
                        let config = Config::from_home_dir(&home_dir);
                        save_config_file(&config_path, &config.serialize()?)?;
                        config
                    }
                };
                Ok(config)
            }
            None => Err(Error::from("Can't get home directory")),
        }
    }

    fn deserialize(file_data: &str) -> Result<Config, Error> {
        Ok(Self {
            work_directory: PathBuf::from(file_data),
        })
    }

    fn serialize(&self) -> Result<String, Error> {
        match &self.work_directory.to_str() {
            Some(res) => Ok(String::from(*res)),
            None => return Err(Error::from("Error while serialize config")),
        }
    }

    fn from_home_dir(home_dir: &Path) -> Self {
        Self {
            work_directory: home_dir.join("code"),
        }
    }
}

fn get_config_file_data(config_path: &Path) -> Result<String, io::Error> {
    let file_path = config_path.join(CONFIG_FILE_NAME);
    let mut file = File::open(&file_path)?;
    let mut file_data = String::new();
    file.read_to_string(&mut file_data)?;
    Ok(file_data)
}

fn save_config_file(config_path: &Path, config: &str) -> Result<(), io::Error> {
    let file_path = config_path.join(CONFIG_FILE_NAME);
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            if !fs::exists(&config_path)? {
                fs::create_dir(&config_path)?;
            }
            File::create(&file_path)?
        }
        Err(e) => return Err(e),
    };
    file.write(config.as_bytes())?;
    Ok(())
}
