use crate::config::{verbosity::Verbosity, Config};

pub struct Logger {
    verbosity: Verbosity,
}

impl Logger {
    pub fn new_for(config: Config) -> Logger {
        Logger {
            verbosity: config.verbosity,
        }
    }
}
