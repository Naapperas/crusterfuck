use std::{
    borrow::{Borrow, BorrowMut},
    fmt,
    path::Path,
    str::Chars,
};

use crate::interpreter::ast::Token;

pub struct Parser {}

pub type ParseResult = Vec<Token>;

pub enum ParseError {
    SourceFileNotFound(String),
    SyntaxError(i16, i16, char),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::SyntaxError(line, col, character) => f.write_fmt(format_args!(
                "Syntax error parsing file contents at line {}, col {}. Found character {}",
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
                Err(ParseError::SourceFileNotFound(
                    path.as_ref().to_str().unwrap().into(), // FIXME: bad unwrap
                ))
            }
        }
    }

    // TODO: make this use a streaming/buffered implementation to be more performant and memory efficient
    pub fn parse(&self, contents: String) -> Result<ParseResult, ParseError> {
        let mut tokens = vec![];

        let mut token_stream = contents.chars();

        let mut line = 1;
        let mut col = 1;

        while let Some(token) = token_stream.next() {
            match token {
                '+' | '-' | '>' | '<' | '.' | ',' => match self.process_op(token) {
                    Some(op_token) => tokens.push(op_token),
                    None => {
                        continue;
                    }
                },
                '[' => {
                    // Start loop construct

                    match self.parse_loop(token_stream.borrow_mut()) {
                        Ok((loop_token, line_delta, col_delta)) => {
                            // update current consumption location
                            if line_delta == 0 {
                                col += col_delta;
                            } else {
                                col = col_delta;
                                line += line_delta;
                            }

                            tokens.push(loop_token)
                        }
                        Err(e) => return Err(e),
                    }
                }
                ']' => {
                    // End loop construct

                    // We should never reach this execution path if the program is syntactically correct (because the loop parsing code consumes the closing bracket).
                    // It is correct to immediately return an error once we get here.

                    return Err(ParseError::SyntaxError(line, col, token));
                }
                '\n' => {
                    line += 1;
                    col = 0;
                }
                _ => {} // other characters are ignored
            }

            col += 1;
        }

        Ok(tokens)
    }

    fn parse_loop(&self, token_stream: &mut Chars) -> Result<(Token, i16, i16), ParseError> {
        let mut loop_tokens = vec![];

        let mut line_delta = 0;
        let mut col_delta = 0;

        loop {
            match token_stream.next() {
                Some(token) => {
                    match token {
                        '+' | '-' | '>' | '<' | '.' | ',' => match self.process_op(token) {
                            Some(op_token) => loop_tokens.push(op_token),
                            None => {
                                continue;
                            }
                        },
                        '[' => {
                            // Start loop construct

                            match self.parse_loop(token_stream) {
                                Ok((loop_token, sub_line_delta, sub_col_delta)) => {
                                    // update current consumption location
                                    if line_delta == 0 {
                                        col_delta += sub_col_delta;
                                    } else {
                                        col_delta = sub_col_delta;
                                        line_delta += sub_line_delta;
                                    }

                                    loop_tokens.push(loop_token)
                                }
                                Err(e) => return Err(e),
                            }
                        }
                        ']' => {
                            // End loop construct

                            // Once we get here we are safe to terminate the loop with the correct loop token generated.

                            return Ok((Token::Loop(loop_tokens), line_delta, col_delta));
                        }
                        '\n' => {
                            line_delta += 1;
                            col_delta = 0;
                        }
                        _ => {} // other characters are ignored
                    }

                    col_delta += 1;
                }
                None => {
                    // We reached the end of the character stream while processing a loop, error
                    return Err(ParseError::SyntaxError(
                        line_delta,
                        col_delta,
                        Default::default(),
                    ));
                }
            }
        }
    }

    fn process_op(&self, token: char) -> Option<Token> {
        match token {
            '+' => Some(Token::Inc),
            '-' => Some(Token::Dec),
            '>' => Some(Token::MoveRight),
            '<' => Some(Token::MoveLeft),
            '.' => Some(Token::Print),
            ',' => Some(Token::Read),
            _ => None,
        }
    }
}
