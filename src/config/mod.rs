pub mod err;
pub mod verbosity;

use err::ConfigErr;
use verbosity::Verbosity;

use serde::Deserialize;
use std::{fs, path::Path};
use toml;

#[derive(Default, Deserialize)]
pub struct Config {
    pub verbosity: Verbosity,
}

pub static CONFIG_FILE_PATH: &'static str = "~/.config/crusterfuck/config.toml";

pub fn parse<P>(path: P) -> Result<Config, ConfigErr>
where
    P: AsRef<Path>,
{
    match fs::read_to_string(path) {
        Ok(contents) => match toml::from_str(&contents) {
            Err(_) => Err(ConfigErr::InvalidConfigValue("test".into(), "test".into())),
            Ok(config) => Ok(config),
        },
        Err(_) => Ok(Default::default()),
    }
}
