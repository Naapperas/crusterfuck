use std::{borrow::Borrow, fmt, path::Path};

use crate::interpreter::{ast::Token, stack::Stack};

pub struct Parser {}

pub type ParseResult = Vec<Token>;

pub enum ParseError {
    SourceFileNotFound(String),
    BadToken(i16, i16, char),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::BadToken(line, col, character) => f.write_fmt(format_args!(
                "Error parsing file contents at line {}, col {}. Found character {}",
                line, col, character
            )),
            ParseError::SourceFileNotFound(path) => f.write_fmt(format_args!(
                "Could not open file at {}: file not found",
                path
            )),
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse_file<P>(&self, path: P) -> Result<ParseResult, ParseError>
    where
        P: AsRef<Path>,
    {
        let res = std::fs::read_to_string(path.borrow());

        match res {
            Ok(contents) => self.parse(contents),
            Err(_) => {
                return Err(ParseError::SourceFileNotFound(
                    path.as_ref().to_str().unwrap().into(), // TODO: bad unwrap
                ));
            }
        }
    }

    // TODO: make this use a streaming/buffered implementation to be more performant
    pub fn parse(&self, contents: String) -> Result<ParseResult, ParseError> {
        let tokens = vec![];

        let token_stream = contents.chars();

        let mut token_stack: Stack<Vec<Token>> = Stack::new();
        token_stack.push(tokens);

        let mut line = 1;
        let mut col = 1;

        for token in token_stream {
            // See how to use peek
            let mut current_level = token_stack.pop().unwrap();

            match token {
                '+' => current_level.push(Token::Inc),
                '-' => current_level.push(Token::Dec),
                '>' => current_level.push(Token::MoveRight),
                '<' => current_level.push(Token::MoveLeft),
                '[' => {
                    // Start loop construct

                    // Loops in brainfuck can be nested, so we need to handle this with a stack structure

                    todo!()
                }
                ']' => {
                    // End loop construct

                    todo!()
                }
                '.' => current_level.push(Token::Print),
                ',' => current_level.push(Token::Read),
                '\n' => {
                    line += 1;
                    col = 0;
                }
                _ => col += 1,
            }

            token_stack.push(current_level);
        }

        if token_stack.is_empty() {
            // we should have at least one single token vector: the program itself

            return Err(ParseError::BadToken(-1, -1, Default::default()));
        } else if token_stack.length() > 1 {
            // if we have more than one token vector that means that we have an unclosed loop somewhere

            return Err(ParseError::BadToken(line, col, Default::default()));
        }

        Ok(token_stack.pop().unwrap())
    }
}
