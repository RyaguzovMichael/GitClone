use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

const CONFIG_FILE_NAME: &str = "config";

pub struct Config {
    pub work_directory: PathBuf,
}

pub struct Error {
    pub message: String,
}

impl Config {
    pub fn default() -> Option<Config> {
        Some(Config {
            work_directory: home::home_dir()?.join("code"),
        })
    }

    pub fn load() -> Result<Config, Error> {
        let home_directory = match home::home_dir() {
            Some(dir) => String::from(dir.to_str().unwrap()),
            None => return Err(Error::from("Can't get home directory")),
        };
        let config_path = Path::new(&home_directory).join(".config").join("gclone");
        let mut file = get_config_file(&config_path)?;
        let mut file_data = String::new();
        file.read_to_string(&mut file_data)?;
        Ok(Config::parse(&file_data.trim())?)
    }

    fn parse(file_data: &str) -> Result<Config, Error> {
        Ok(Self {
            work_directory: PathBuf::from(file_data),
        })
    }
}

fn get_config_file(config_path: &PathBuf) -> Result<File, Error> {
    if !fs::exists(&config_path)? {
        fs::create_dir(&config_path)?;
    }
    let file_path = config_path.join(CONFIG_FILE_NAME);
    if !fs::exists(&file_path)? {
        let mut file = File::create(&file_path)?;
        let config = Config::default().unwrap();
        let config = match config.work_directory.to_str() {
            Some(res) => res,
            None => return Err(Error::from("Can't create work directory default")),
        };
        file.write(config.as_bytes())?;
        Ok(file)
    } else {
        Ok(File::open(&file_path)?)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl Error {
    fn from(error_message: &str) -> Self {
        Self {
            message: String::from(error_message),
        }
    }
}
