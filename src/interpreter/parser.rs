use std::fmt;

use crate::interpreter::{ast::Token, stack::Stack};

pub struct Parser {}

pub type ParseResult = Vec<Token>;

pub struct ParseError {
    line: i32,
    col: i32,
    character: char,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Error parsing file contents at line {}, col {}. Found character {}",
            self.line, self.col, self.character
        ))
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

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
                _ => {
                    return Err(ParseError {
                        line,
                        col,
                        character: token,
                    })
                }
            }

            token_stack.push(current_level);
        }

        if token_stack.is_empty() {
            // we should have at least one single token vector: the program itself

            return Err(ParseError {
                line: -1,
                col: -1,
                character: Default::default(),
            });
        } else if token_stack.length() > 1 {
            // if we have more than one token vector that means that we have an unclosed loop somewhere

            return Err(ParseError {
                line,
                col,
                character: Default::default(),
            });
        }

        Ok(token_stack.pop().unwrap())
    }
}
