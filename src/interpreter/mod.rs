use self::{ast::Token, io::IO};

pub mod ast;
mod io;
pub mod parser;
mod stack;

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

    // TODO: see if we need to expose a non-mutable API
    pub fn run(&mut self, tokens: parser::ParseResult) -> Result<(), ProgramError> {
        for token in tokens {
            match token {
                Token::Inc => self.data[self.pointer] += 1,
                Token::Dec => self.data[self.pointer] -= 1,
                Token::MoveLeft => self.pointer -= 1,
                Token::MoveRight => self.pointer += 1,
                Token::Print => {
                    let current_byte = self.data[self.pointer];

                    let character = current_byte as char;

                    self.io.print(character);
                }
                Token::Read => {
                    let character = self.io.read();

                    self.data[self.pointer] = character as u8;
                }
                Token::Loop(loop_tokens) => todo!(),
            }
        }

        Ok(())
    }
}
