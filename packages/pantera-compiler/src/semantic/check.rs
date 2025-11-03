use crate::errors::CompilerError;

pub trait Check {
    fn get_errors(self) -> Vec<CompilerError>;
}