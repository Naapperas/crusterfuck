use std::fmt;

pub enum ConfigErr {
    InvalidConfigValue(String, String),
}

impl fmt::Display for ConfigErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message: String = match self {
            ConfigErr::InvalidConfigValue(path, value) => {
                format!("Invalid value found at {path}: {value}")
            }
        };

        write!(f, "{message}")
    }
}
