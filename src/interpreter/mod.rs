use self::{ast::Token, io::IO};

pub mod ast;
mod io;
pub mod parser;

pub struct Interpreter {
    data: [u8; 30000],
    pointer: usize,
    io: IO,
}

pub struct ProgramError {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            data: [0; 30000],
            pointer: 0,
            io: IO::new(),
        }
    }

    fn process_token(&mut self, token: Token) -> Result<(), ProgramError> {
        match token {
            Token::Inc => self.data[self.pointer] += 1,
            Token::Dec => self.data[self.pointer] -= 1,
            Token::MoveLeft => self.pointer -= 1, // TODO: bounds checks
            Token::MoveRight => self.pointer += 1, // TODO: bounds checks
            Token::Print => {
                let current_byte = self.data[self.pointer];

                let character = current_byte as char;

                self.io.print(character);
            }
            Token::Read => {
                let character = self.io.read();

                self.data[self.pointer] = character as u8;
            }
            Token::Loop(loop_tokens) => {
                if let Err(err) = self.run_loop(loop_tokens) {
                    return Err(err);
                }
            }
        };

        Ok(())
    }

    // TODO: see if we need to expose a non-mutable API
    pub fn run(&mut self, tokens: parser::ParseResult) -> Result<(), ProgramError> {
        for token in tokens {
            if let Err(err) = self.process_token(token) {
                return Err(err);
            }
        }

        Ok(())
    }

    fn run_loop(&mut self, loop_tokens: Vec<Token>) -> Result<(), ProgramError> {
        loop {
            if self.data[self.pointer] == 0 {
                break;
            }

            for token in loop_tokens.clone() {
                if let Err(err) = self.process_token(token) {
                    return Err(err);
                }
            }
        }

        Ok(())
    }
}
