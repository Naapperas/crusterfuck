use std::fmt;

/// Error generated when parsing a configuration file.
pub enum ConfigErr {
    /// A key has a corresponding value that is invalid.
    InvalidConfigValue { path: String, value: String },
}

impl fmt::Display for ConfigErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message: String = match self {
            ConfigErr::InvalidConfigValue { path, value } => {
                format!("Invalid value found at {path}: {value}")
            }
        };

        write!(f, "{message}")
    }
}
