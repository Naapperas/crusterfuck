use std::fmt;

use crate::config::{verbosity::VerbosityLevel, Config};

/// Simple implementation of logging to `stdout` according to predefined logging verbosity levels.
pub struct Logger<'a> {
    config: &'a Config,
}

/// Support macro useful for repeating the same logging logic with varying logging levels.
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
    /// Constructs a new [Logger] with the specified configuration
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
