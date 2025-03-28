use crate::bytecode::Bytecode;

pub enum Type {
    Null = 0,
    Number = 1,
    Boolean = 2
}

impl Into<Bytecode> for Type {
    fn into(self) -> Bytecode {
        match self {
            Type::Null => 0,
            Type::Number => 1,
            Type::Boolean => 2
        }
    }
}

impl Type {
    pub fn from(byte: Bytecode) -> Self {
        match byte {
            0 => Type::Null,
            1 => Type::Number,
            2 => Type::Boolean,
            _ => panic!("Type doesn't exist")
        }
    }


}