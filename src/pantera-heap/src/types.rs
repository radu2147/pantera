pub enum Type {
    Null = 0,
    Number = 1,
    Boolean = 2,
    Function = 3,
    String = 4,
    Object = 5
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        match value {
            0 => Type::Null,
            1 => Type::Number,
            2 => Type::Boolean,
            3 => Type::Function,
            4 => Type::String,
            5 => Type::Object,
            _ => panic!("Type doesn't exist")
        }
    }
}