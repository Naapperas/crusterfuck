use std::process::exit;

use clap::Parser;

mod cli;
mod config;
mod interpreter;
mod log;

fn main() {
    let args = cli::CommandLineArgs::parse();

    let config = match config::parse(args.config) {
        Ok(config) => config,
        Err(error) => match error {
            config::err::ConfigErr::InvalidConfigValue(_, _) => todo!(),
        },
    };

    let logger = log::Logger::new_for(config);

    let parser = interpreter::parser::Parser::new();

    let tokens = match parser.parse(",.".into()) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err);

            exit(1);
        }
    };

    let mut interpreter = interpreter::Interpreter::new();

    if let Err(err) = interpreter.run(tokens) {
        exit(1); // TODO: better error handling
    }
}
