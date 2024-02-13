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
        Err(error) => {
            eprintln!("{}", error);

            exit(1)
        }
    };

    let logger = log::Logger::new_for(&config);

    let parser = interpreter::parser::Parser::new();

    logger.medium(format!("Parsing file: {}", args.input));

    let tokens = match parser.parse_file(args.input) {
        Ok(tokens) => tokens,
        Err(err) => {
            logger.high(format!("Error parsing file: {}", err));

            exit(2);
        }
    };

    logger.medium("Running Brainfuck program");

    let interpreter = interpreter::Interpreter::new();

    if let Err(err) = interpreter.run(tokens) {
        logger.high(format!("Error while running Brainfuck code: {}", err));

        exit(3);
    }
}
