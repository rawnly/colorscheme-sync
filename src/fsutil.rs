use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

use crate::error;

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
