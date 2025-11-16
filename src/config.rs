use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gemini_api_key: String,
}

impl Config {
    pub fn load() -> Option<Self> {
        let path = config_path();
        if path.exists() {
            let data = fs::read_to_string(path).ok()?;
            toml::from_str(&data).ok()
        } else {
            None
        }
    }

    pub fn save(key: &str) {
        let cfg = Config {
            gemini_api_key: key.to_string(),
        };
        let path = config_path();
        let dir = path.parent().unwrap();
        fs::create_dir_all(dir).unwrap();
        fs::write(path, toml::to_string(&cfg).unwrap()).unwrap();
    }
}

pub fn config_path() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(".mgit/config.toml");
    path
}
