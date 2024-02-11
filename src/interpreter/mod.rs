use std::fmt;

use self::{ast::Token, io::IO};

pub mod ast;
mod io;
pub mod parser;

const BUFFER_SIZE: usize = 30000;

pub struct Interpreter {
    data: [u8; BUFFER_SIZE],
    pointer: usize,
    io: IO,
}

pub enum ProgramError {
    OutOfBounds,
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramError::OutOfBounds => write!(f, "Attempted to move pointer past end of buffer"),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            data: [0; BUFFER_SIZE],
            pointer: 0,
            io: IO::new(),
        }
    }

    fn process_token(&mut self, token: Token) -> Result<(), ProgramError> {
        match token {
            Token::Inc => self.data[self.pointer] += 1,
            Token::Dec => self.data[self.pointer] -= 1,
            Token::MoveLeft => {
                if self.pointer == 0 {
                    return Err(ProgramError::OutOfBounds);
                }

                self.pointer -= 1;
            }
            Token::MoveRight => {
                if self.pointer == BUFFER_SIZE - 1 {
                    return Err(ProgramError::OutOfBounds);
                }

                self.pointer += 1;
            }
            Token::Print => {
                let current_byte = self.data[self.pointer];

                let character = current_byte as char;

                self.io.print(character);
            }
            Token::Read => {
                let character = self.io.read();

                self.data[self.pointer] = character as u8;
            }
            Token::Loop(loop_tokens) => self.run_loop(loop_tokens)?,
        };

        Ok(())
    }

    // TODO: see if we need to expose a non-mutable API
    pub fn run(&mut self, tokens: parser::ParseResult) -> Result<(), ProgramError> {
        for token in tokens {
            self.process_token(token)?
        }

        Ok(())
    }

    fn run_loop(&mut self, loop_tokens: Vec<Token>) -> Result<(), ProgramError> {
        loop {
            if self.data[self.pointer] == 0 {
                break;
            }

            for token in loop_tokens.clone() {
                self.process_token(token)?
            }
        }

        Ok(())
    }
}
