use std::io::{stdin, Read};
use std::str;

/// Abstraction over IO operations that can be made.
pub struct IO {}

impl IO {
    /// Constructs a new [IO] object.
    pub fn new() -> Self {
        IO {}
    }

    /// Prints [c] to the output stream defined by this [IO] object.
    ///
    /// By default, the output stream will be [stdout].
    pub fn print(&self, c: char) {
        print!("{c}")
    }

    /// Reads exactly one character from `stdin` and returns it as an [Option]
    ///
    /// Returns [None] if an error occurs when attempting to read from `stdin` or convert the input.
    pub fn read(&self) -> Option<char> {
        let mut byte = [0_u8];

        let mut stdin_handle = stdin().lock();

        match stdin_handle.read_exact(&mut byte) {
            Ok(_) => match str::from_utf8(&mut byte) {
                Ok(s) => s.chars().nth(0),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}
