use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

use etcetera::{choose_app_strategy, AppStrategy, AppStrategyArgs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub last_bear: Option<PathBuf>,
}

impl Config {
    pub fn save(&self) {
        let mut path = Self::path();
        path.pop();
        std::fs::create_dir_all(path).unwrap();

        write!(
            File::create(Self::path()).unwrap(),
            "{}",
            toml::to_string_pretty(self).unwrap(),
        )
        .unwrap();
    }

    pub fn load() -> Self {
        read_to_string(Self::path())
            .ok()
            .and_then(|contents| toml::from_str(&contents).ok())
            .unwrap_or_else(|| {
                let to_ret = Self::default();
                to_ret.save();
                to_ret
            })
    }

    pub fn path() -> PathBuf {
        choose_app_strategy(AppStrategyArgs {
            top_level_domain: "org".to_owned(),
            author: "SalsaGal".to_owned(),
            app_name: "HoneyHeist".to_owned(),
        })
        .unwrap()
        .in_config_dir("config.toml")
    }
}
