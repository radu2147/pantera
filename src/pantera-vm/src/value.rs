use std::fmt::{Display, Formatter, Write};
use pantera_compiler::heap::{HeapManager, HeapValue, Ptr};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Null,
    Function(usize, u8),
    String(Ptr)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => f.write_str(&num.to_string()),
            Self::Null => f.write_str("null"),
            Self::Bool(val) => f.write_str(&val.to_string()),
            Self::Function(_, _) => f.write_str("[function]"),
            Self::String(ptr) => {
                let HeapValue::String(str) = HeapManager::get_object(ptr.clone()) else {panic!("Not implemented")};
                f.write_str(&str)
            }
        }
    }
}