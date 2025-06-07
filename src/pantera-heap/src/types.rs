pub enum Type {
    Empty = 0,
    Number = 1,
    Boolean = 2,
    Function = 3,
    String = 4,
    Object = 5,
    Null = 6
}

impl Into<u8> for Type {
    fn into(self) -> u8 {
        match self {
            Type::Empty => 0u8,
            Type::Number => 1u8,
            Type::Boolean => 2u8,
            Type::Function => 3u8,
            Type::String => 4u8,
            Type::Object => 5u8,
            Type::Null => 6u8
        }
    }
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        match value {
            6 => Type::Null,
            1 => Type::Number,
            2 => Type::Boolean,
            3 => Type::Function,
            4 => Type::String,
            5 => Type::Object,
            0 => Type::Empty,
            _ => panic!("Type doesn't exist")
        }
    }
}