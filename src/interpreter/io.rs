use std::io::{stdin, Read};
use std::str;

pub struct IO {}

impl IO {
    pub fn new() -> Self {
        IO {}
    }

    pub fn print(&self, c: char) {
        print!("{c}")
    }

    pub fn read(&self) -> Option<char> {
        // TODO: I don't like this that much but oh well
        let mut byte = [0_u8];

        let mut stdin_handle = stdin().lock();

        stdin_handle.read_exact(&mut byte).ok()?;

        match str::from_utf8(&byte) {
            Ok(v) => v.chars().nth(0),
            Err(e) => panic!("Non UTF-8 character read: {}", e),
        }
    }
}
