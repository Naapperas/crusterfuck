use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineArgs {
    #[arg(short, long, default_value_t = crate::config::CONFIG_FILE_PATH.to_string())]
    pub config: String,

    pub input: String,
}
