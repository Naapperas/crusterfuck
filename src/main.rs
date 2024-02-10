use crate::config::CONFIG_FILE_PATH;

mod config;
mod interpreter;
mod io;
mod stack;

fn main() {
    let config = match config::parse(CONFIG_FILE_PATH) {
        Ok(config) => config,
        Err(error) => match error {
            config::err::ConfigErr::InvalidConfigValue(_, _) => panic!(),
        },
    };

    println!("Hello, world!");
}
