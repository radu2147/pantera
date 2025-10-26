#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: u128,
}