use std::fmt;

use crate::config::{verbosity::VerbosityLevel, Config};

pub struct Logger<'a> {
    config: &'a Config,
}

macro_rules! _log {
    ($name:ident, $levels:pat) => {
        pub fn $name<M>(&self, message: M)
        where
            M: fmt::Display,
        {
            if self.config.verbosity.enabled {
                return;
            }

            match self.config.verbosity.level {
                $levels => {
                    println!("{}", message);
                }
                _ => {}
            }
        }
    };
}

impl<'a> Logger<'a> {
    pub fn new_for(config: &'a Config) -> Logger {
        Logger { config }
    }

    _log!(high, VerbosityLevel::High);

    _log!(medium, VerbosityLevel::High | VerbosityLevel::Medium);

    _log!(
        low,
        VerbosityLevel::High | VerbosityLevel::Medium | VerbosityLevel::Low
    );
}
