pub enum Type {
    Empty = 0,
    Number = 1,
    Boolean = 2,
    Function = 3,
    String = 4,
    Object = 5,
    Array = 6,
    Null = 7
}

impl From<Type> for u8 {
    fn from(val: Type) -> Self {
        match val {
            Type::Empty => 0u8,
            Type::Number => 1u8,
            Type::Boolean => 2u8,
            Type::Function => 3u8,
            Type::String => 4u8,
            Type::Object => 5u8,
            Type::Array => 6u8,
            Type::Null => 7u8
        }
    }
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        match value {
            1 => Type::Number,
            2 => Type::Boolean,
            3 => Type::Function,
            4 => Type::String,
            5 => Type::Object,
            6 => Type::Array,
            7 => Type::Null,
            0 => Type::Empty,
            _ => panic!("Type doesn't exist")
        }
    }
}