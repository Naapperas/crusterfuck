use clap::Parser;

/// Rust-based brainfuck interpreter.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineArgs {
    /// The path of the configuration file.
    #[arg(short, long, default_value_t = crate::config::CONFIG_FILE_PATH.to_string())]
    pub config: String,

    /// The path of the source file to interpret.
    pub input: String,
}
