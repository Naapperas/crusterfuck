use std::io::{stdin, Read};
use std::str;

pub struct IO {}

impl IO {
    pub fn new() -> Self {
        IO {}
    }

    pub fn print(&self, c: char) {
        println!("{c}")
    }

    pub fn read(&self) -> char {
        // TODO: I don't like this that much but oh well

        let mut stdin_handle = stdin().lock();
        let mut byte = [0_u8];
        stdin_handle.read_exact(&mut byte).unwrap();

        match str::from_utf8(&byte) {
            Ok(v) => v.chars().nth(0).unwrap(),
            Err(e) => panic!("Non UTF-8 character read: {}", e),
        }
    }
}
