use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub enum VerbosityLevel {
    #[default]
    None,

    Low,

    Medium,

    High,
}

#[derive(Default, Deserialize)]
pub struct Verbosity {
    pub enabled: bool,
    pub level: VerbosityLevel,
}
