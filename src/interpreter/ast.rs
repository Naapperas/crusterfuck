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
