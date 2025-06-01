use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use crate::heap::{HeapManager, HeapValue, Ptr};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Null,
    Function(usize, u8),
    String(Ptr),
    Object(Ptr)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => f.write_str(&num.to_string()),
            Self::Null => f.write_str("null"),
            Self::Bool(val) => f.write_str(&val.to_string()),
            Self::Function(_, _) => f.write_str("[function]"),
            Self::String(ptr) => {
                let HeapValue::String(str) = HeapManager::get_from_heap(ptr.clone()) else {panic!("Not implemented")};
                f.write_str(&str)
            },
            Self::Object(ptr) => {
                let HeapValue::Object(obj) = HeapManager::get_from_heap(ptr.clone()) else {panic!("Not implemented")};
                let mut str = String::new();
                str = str.add("{ ");
                let mut pairs = vec![];
                for (key, val) in obj {
                    let mut pair = String::new();
                    pair = pair.add(&key);
                    pair = pair.add(": ");
                    pair = pair.add(&format!("{}", val));
                    pairs.push(pair);
                }
                str = str.add(pairs.join(", ").as_str());
                str = str.add(" }");

                f.write_str(&str)
            },
        }
    }
}