use std::fmt;

/// Token that is parsed by a [Parser] from source input and can be consumed by an [Interpreter].
#[derive(Debug, Clone)]
pub enum Token {
    /// Signals the [Interpreter] to increase the value at the currently pointed at location by 1.
    Inc,

    /// Signals the [Interpreter] to decrease the value at the currently pointed at location by 1.
    Dec,

    /// Signals the [Interpreter] to move the pointer one place to the right.
    MoveRight,

    /// Signals the [Interpreter] to move the pointer one place to the left.
    MoveLeft,

    /// Signals the [Interpreter] to run the given list of commands in a loop.
    Loop(Vec<Token>),

    /// Signals the [Interpreter] to print to `stdout` the value at the currently pointed at location.
    Print,

    /// Signals the [Interpreter] to read a byte from `stdin` and store it at the currently pointed at location.
    Read,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inc => write!(f, "Inc"),
            Self::Dec => write!(f, "Dec"),
            Self::MoveRight => write!(f, "MoveRght"),
            Self::MoveLeft => write!(f, "MoveLeft"),
            Self::Loop(vec) => {
                write!(f, "Loop[")?;

                for ele in vec {
                    write!(f, "{},", ele)?;
                }

                return write!(f, "]");
            }
            Self::Print => write!(f, "Print"),
            Self::Read => write!(f, "Read"),
        }
    }
}
