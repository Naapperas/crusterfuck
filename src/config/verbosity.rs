use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub enum VerbosityLevel {
    #[default]
    NONE,

    LOW,

    MEDIUM,

    HIGH,
}

#[derive(Default, Deserialize)]
pub struct Verbosity {
    enabled: bool,
    level: VerbosityLevel,
}
