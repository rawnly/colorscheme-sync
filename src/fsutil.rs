use home::home_dir;
use serde::de::DeserializeOwned;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use crate::error;
use crate::models::Theme;

pub trait File
where
    Self: DeserializeOwned + serde::Serialize,
{
    fn load(path: &Path) -> crate::Result<Self> {
        let file = fs::File::open(path)?;

        serde_json::from_reader(file).map_err(error::Error::JSON)
    }

    fn write(&self, path: &Path) -> crate::Result<()> {
        let file = fs::File::create(path)?;

        serde_json::to_writer_pretty(file, self).map_err(error::Error::JSON)
    }
}

/// Search for a valid config file in the following locations
/// - $XDG_CONFIG/colorscheme/themes.json
/// - ~/.colorscheme/themes.json
/// - ~/.colorscheme.json
pub fn get_config_file() -> crate::Result<PathBuf> {
    let config_path = get_config_dir_file();
    let home_config_path = get_home_file();

    if !config_path.exists() {
        if !home_config_path.exists() {
            // create a new file inside `config_path`
            let themes = Vec::<Theme>::new();
            themes.write(config_path.as_path())?;

            return Ok(config_path);
        }

        // read from home
        return Ok(home_config_path);
    }

    Ok(config_path)
}

pub fn get_config_path() -> PathBuf {
    let home = home_dir().expect("Unable to retrive homedir.");

    match env::var("XDG_CONFIG_HOME") {
        Ok(p) => PathBuf::from_str(&p).unwrap(),
        Err(_) => {
            let home = home.join(Path::new(".config"));
            home
        }
    }
}

fn get_config_dir_file() -> PathBuf {
    let config_path = get_config_path();
    config_path.join("colorscheme/themes.json")
}

fn get_home_file() -> PathBuf {
    let home = home_dir().expect("Unable to retrive homedir.");
    home.join(".colorscheme.json")
}

// inspired by official alacritty repo
pub fn read_yaml(path: &Path) -> crate::Result<serde_yaml::Value> {
    let mut content = fs::read_to_string(path)?;

    // Remove UTF-8 BOM.
    if content.starts_with('\u{FEFF}') {
        content = content.split_off(3);
    }

    // load configuration file as Value
    let config = match serde_yaml::from_str(&content) {
        Ok(value) => value,
        Err(error) => {
            if error.to_string() == "EOF while parsing a value" {
                serde_yaml::Value::Mapping(serde_yaml::Mapping::new())
            } else {
                return Err(error::Error::YAML(error));
            }
        }
    };

    Ok(config)
}
