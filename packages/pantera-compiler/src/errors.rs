#[derive(Debug, PartialEq)]
pub struct CompilerError {
    pub message: String,
}

impl CompilerError {
    pub fn get_message(&self) -> String {
        String::from(format!("Compiler Error: {}", self.message))
    }
}