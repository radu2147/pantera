use crate::types::Type;

pub type Bytecode = u8;

impl Into<Type> for &Bytecode {
    fn into(self) -> Type {
        match self {
            0 => Type::Null,
            1 => Type::Number,
            2 => Type::Boolean,
            3 => Type::Function,
            4 => Type::String,
            _ => panic!("Type not supported")
        }
    }
}

macro_rules! generate_bytecode {
    // Entry point: take first separately
    ($first:ident $(, $rest:ident)*) => {
        generate_bytecode!(@internal $first; $($rest),*);
    };

    // Internal recursive pattern
    (@internal $prev:ident; $next:ident $(, $rest:ident)*) => {
        pub const $next :Bytecode = $prev + 1;
        generate_bytecode!(@internal $next; $($rest),*);
    };

    // Base case
    (@internal $_prev:ident;) => {};
}

pub const START_OP_CODE: Bytecode = 0; // exclusive

generate_bytecode! (
    START_OP_CODE,
    OP_PRINT,
    OP_ADD,
    OP_SUB,
    OP_DIV,
    OP_MUL,
    OP_POW,
    OP_POP,
    OP_PUSH,
    OP_EQ,
    OP_NE,
    OP_AND,
    OP_OR,
    OP_GE,
    OP_GR,
    OP_LE,
    OP_LS,
    OP_UNARY_SUB,
    OP_UNARY_NOT,
    OP_DECLARE,
    OP_DECLARE_GLOBAL,
    OP_GET,
    OP_GET_GLOBAL,
    OP_SET,
    OP_SET_GLOBAL,
    OP_JUMP_IF_FALSE,
    OP_JUMP,
    OP_END_FUNCTION,
    OP_CALL,
    OP_RETURN
);