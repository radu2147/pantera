use crate::types::Type;

pub type Bytecode = u8;

impl Into<Type> for &Bytecode {
    fn into(self) -> Type {
        match self {
            0 => Type::Null,
            1 => Type::Number,
            2 => Type::Boolean,
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
    OP_GET
);