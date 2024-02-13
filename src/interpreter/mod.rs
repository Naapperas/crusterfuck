use std::{cell::RefCell, fmt};

use self::{ast::Token, io::IO};

pub mod ast;
mod io;
pub mod parser;

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

const BUFFER_SIZE: usize = 30000;

struct ArrayBuffer {
    data: [u8; BUFFER_SIZE],
    pointer: usize,
}

impl ArrayBuffer {
    fn new() -> Self {
        ArrayBuffer {
            data: [0; BUFFER_SIZE],
            pointer: 0,
        }
    }

    fn pointer(&self) -> usize {
        self.pointer
    }

    fn get(&self) -> u8 {
        self.data[self.pointer]
    }

    fn set(&mut self, value: u8) {
        self.data[self.pointer] = value;
    }

    fn inc(&mut self) {
        self.data[self.pointer] += 1;
    }

    fn dec(&mut self) {
        self.data[self.pointer] -= 1;
    }

    fn pointer_left(&mut self) {
        self.pointer -= 1;
    }

    fn pointer_right(&mut self) {
        self.pointer += 1;
    }
}

pub struct Interpreter {
    data: RefCell<ArrayBuffer>,
    io: IO,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            data: RefCell::new(ArrayBuffer::new()),
            io: IO::new(),
        }
    }

    fn process_token(&self, token: Token) -> Result<(), ProgramError> {
        match token {
            Token::Inc => self.data.borrow_mut().inc(),
            Token::Dec => self.data.borrow_mut().dec(),
            Token::MoveLeft => {
                if self.data.borrow_mut().pointer() == 0 {
                    return Err(ProgramError::OutOfBounds);
                }

                self.data.borrow_mut().pointer_left();
            }
            Token::MoveRight => {
                if self.data.borrow_mut().pointer() == BUFFER_SIZE - 1 {
                    return Err(ProgramError::OutOfBounds);
                }

                self.data.borrow_mut().pointer_right();
            }
            Token::Print => {
                let current_byte = self.data.borrow_mut().get();

                let character = current_byte as char;

                self.io.print(character);
            }
            Token::Read => {
                let character = self.io.read();

                if character.is_none() {
                    return Err(ProgramError::OutOfBounds);
                }

                self.data.borrow_mut().set(character.unwrap() as u8);
            }
            Token::Loop(loop_tokens) => self.run_loop(loop_tokens)?,
        };

        Ok(())
    }

    pub fn run(&self, tokens: parser::ParseResult) -> Result<(), ProgramError> {
        for token in tokens {
            self.process_token(token)?
        }

        Ok(())
    }

    fn run_loop(&self, loop_tokens: Vec<Token>) -> Result<(), ProgramError> {
        loop {
            if self.data.borrow_mut().get() == 0 {
                break;
            }

            for token in loop_tokens.clone() {
                self.process_token(token)?
            }
        }

        Ok(())
    }
}
