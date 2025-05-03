use crate::bytecode::Bytecode;

pub enum Type {
    Null = 0,
    Number = 1,
    Boolean = 2,
    Function = 3,
    String = 4
}

impl Into<Bytecode> for Type {
    fn into(self) -> Bytecode {
        match self {
            Type::Null => 0,
            Type::Number => 1,
            Type::Boolean => 2,
            Type::Function => 3,
            Type::String => 4
        }
    }
}

impl From<Bytecode> for Type {
    fn from(value: Bytecode) -> Self {
        match value {
            0 => Type::Null,
            1 => Type::Number,
            2 => Type::Boolean,
            3 => Type::Function,
            4 => Type::String,
            _ => panic!("Type doesn't exist")
        }
    }
}