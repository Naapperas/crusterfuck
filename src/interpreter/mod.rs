use std::{cell::RefCell, fmt};

use self::{ast::Token, io::IO};

pub mod ast;
mod io;
pub mod parser;

/// Runtime error when running the interpreter.
pub enum ProgramError {
    /// Attempted to move the array pointer out of the array buffer's bounds.
    OutOfBounds,

    /// Error when performing an I/O operation.
    IOError,

    /// Error when performing illegal operation
    IllegalOperation 
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProgramError::OutOfBounds => write!(f, "Attempted to move pointer past end of buffer"),
            ProgramError::IOError => write!(f, "Error when attempting to perform IO operation"),
            ProgramError::IllegalOperation => write!(f, "Operation is illegal")
        }
    }
}

/// Max size of the buffer in bytes.
const BUFFER_SIZE: usize = 30000;

/// Buffer used internally by the interpreter.
///
/// This struct exists for the sole purpose of exploiting **interior mutability**, allowing [Interpreter] objects to expose an immutable API.
struct ArrayBuffer {
    /// The data held in the buffer.
    data: [u8; BUFFER_SIZE],

    /// A pointer to the *current location* in the buffer.
    pointer: usize,
}

impl ArrayBuffer {
    /// Creates a new empty [ArrayBuffer] with all elements set to 0 which points to the byte at position 0.
    fn new() -> Self {
        ArrayBuffer {
            data: [0; BUFFER_SIZE],
            pointer: 0,
        }
    }

    /// Returns the value of the pointer.
    fn pointer(&self) -> usize {
        self.pointer
    }

    /// Returns the value currently pointed at in the buffer.
    fn get(&self) -> u8 {
        self.data[self.pointer]
    }

    /// Sets the position currently pointed at in the buffer to hold *value*.
    fn set(&mut self, value: u8) {
        self.data[self.pointer] = value;
    }

    /// Increments by 1 the value currently pointed at in the buffer
    fn inc(&mut self) {
        self.data[self.pointer] += 1;
    }

    /// Decrements by 1 the value currently pointed at in the buffer
    fn dec(&mut self) {
        self.data[self.pointer] -= 1;
    }

    /// Moves the pointer one position to the left.
    fn pointer_left(&mut self) {
        self.pointer -= 1;
    }

    /// Moves the pointer one position to the right.
    fn pointer_right(&mut self) {
        self.pointer += 1;
    }
}

/// A interpreting engine for [ParseResult] responsible for consuming the parsed input and simulating the respective code.
pub struct Interpreter {
    /// The data used by the interpreter.
    ///
    /// This is stored behind a [RefCell] so we can expose an immutable API.
    data: RefCell<ArrayBuffer>,

    /// Object that abstracts I/O operations away from interpreter objects.
    io: IO,
}

impl Interpreter {
    /// Creates a new [Interpreter] object with a empty [buffer](ArrayBuffer).
    pub fn new() -> Self {
        Interpreter {
            data: RefCell::new(ArrayBuffer::new()),
            io: IO::new(),
        }
    }

    /// Processes the given [Token] and returns the result of its computation.
    fn process_token(&self, token: Token) -> Result<(), ProgramError> {
        match token {
            Token::Inc => {
                self.data.borrow_mut().inc();
            }
            Token::Dec => {
                if self.data.borrow().get() == 0 {
                    return Err(ProgramError::IllegalOperation);
                }
                
                self.data.borrow_mut().dec();
            }
            Token::MoveLeft => {
                if self.data.borrow().pointer() == 0 {
                    return Err(ProgramError::OutOfBounds);
                }

                self.data.borrow_mut().pointer_left();
            }
            Token::MoveRight => {
                if self.data.borrow().pointer() == BUFFER_SIZE - 1 {
                    return Err(ProgramError::OutOfBounds);
                }

                self.data.borrow_mut().pointer_right();
            }
            Token::Print => {
                let current_byte = self.data.borrow().get();

                let character = current_byte as char;

                self.io.print(character);
            }
            Token::Read => match self.io.read() {
                None => return Err(ProgramError::IOError),
                Some(character) => self.data.borrow_mut().set(character as u8),
            },
            Token::Loop(loop_tokens) => self.run_loop(loop_tokens)?,
        };

        Ok(())
    }

    /// Consumes the input tokens and runs the corresponding program.
    pub fn run(&self, tokens: parser::ParseResult) -> Result<(), ProgramError> {
        for token in tokens {
            self.process_token(token)?
        }

        Ok(())
    }

    /// Specialized function used to run loops.
    fn run_loop(&self, loop_tokens: Vec<Token>) -> Result<(), ProgramError> {
        loop {
            if self.data.borrow().get() == 0 {
                break;
            }

            for token in loop_tokens.clone() {
                self.process_token(token)?
            }
        }

        Ok(())
    }
}
