pub mod err;
pub mod verbosity;

use err::ConfigErr;
use verbosity::Verbosity;

use serde::Deserialize;
use std::{fs, path::Path};

/// Global program configurations.
#[derive(Default, Deserialize)]
pub struct Config {
    /// The verbosity configurations to apply when logging messages.
    pub verbosity: Verbosity,
}

/// The default path for the config file.
pub static CONFIG_FILE_PATH: &str = "~/.config/crusterfuck/config.toml";

/// Reads the contents of the file at the given path and parses them into a [Config] object.
pub fn parse<P>(path: P) -> Result<Config, ConfigErr>
where
    P: AsRef<Path>,
{
    match fs::read_to_string(path) {
        Ok(contents) => match toml::from_str(&contents) {
            Err(_) => Err(ConfigErr::InvalidConfigValue {
                path: "test".into(),
                value: "test".into(),
            }),
            Ok(config) => Ok(config),
        },
        Err(_) => Ok(Default::default()),
    }
}
