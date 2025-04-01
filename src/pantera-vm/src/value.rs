use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Null
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => f.write_str(&num.to_string()),
            Self::Null => f.write_str("null"),
            Self::Bool(val) => f.write_str(&val.to_string())
        }
    }
}