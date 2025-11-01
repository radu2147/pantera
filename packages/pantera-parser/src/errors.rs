#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: u128,
}

impl ParseError {
    pub fn get_message(&self) -> String {
        String::from(format!("Parser Error: {}", self.message))
    }
}