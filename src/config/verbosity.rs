// TODO: this is awkward.

use serde::Deserialize;

/// Level of verbosity when logging messages.
#[derive(Default, Debug, Deserialize)]
pub enum VerbosityLevel {
    /// No messages logged.
    #[default]
    None,

    /// All messages are logged.
    Low,

    /// Only low priority messages are *not* logged.
    Medium,

    /// Only high priority messages are logged.
    High,
}

/// Descriptor of the verbosity configurations for logging messages.
#[derive(Default, Deserialize)]
pub struct Verbosity {
    /// Whether logging is enabled.
    pub enabled: bool,

    /// The level of verbosity to use when logging messages.
    pub level: VerbosityLevel,
}
