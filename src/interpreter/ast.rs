#[derive(Debug)]
pub enum Token {
    Inc,
    Dec,
    MoveRight,
    MoveLeft,
    Loop(Vec<Token>),
    Print,
    Read,
}