use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use crate::heap::{HeapManager, Ptr};

pub enum HeapValue {
    String(String),
    Object(HashMap<String, Box<HeapValue>>),
    Value(Value)
}

impl Display for HeapValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(val) => {
                f.write_str(&val.to_string())
            },
            Self::String(str) => {
                f.write_str(&format!("\"{str}\""))
            },
            Self::Object(obj) => {
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
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Null,
    Function(usize, u8),
    String(Ptr),
    Object(Ptr)
}

impl Into<HeapValue> for Value {
    fn into(self) -> HeapValue {
        HeapValue::Value(self)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => f.write_str(&num.to_string()),
            Self::Null => f.write_str("null"),
            Self::Bool(val) => f.write_str(&val.to_string()),
            Self::Function(_, _) => f.write_str("[function]"),
            Self::String(ptr) => {
                let str = HeapManager::get_from_heap(ptr.clone());
                f.write_str(&str.to_string())
            },
            Self::Object(ptr) => {
                let obj = HeapManager::get_from_heap(ptr.clone());
                f.write_str(&obj.to_string())
            },
        }
    }
}