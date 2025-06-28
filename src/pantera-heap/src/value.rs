use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use crate::heap::{HeapManager, Ptr};

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
                let str = HeapManager::get_string(*ptr);
                f.write_str(&str.to_string())
            },
            Self::Object(obj_ptr) => {
                unsafe {
                    let obj = HeapManager::get_object(*obj_ptr);
                    let mut str = String::new();
                    str = str.add("{ ");
                    let mut pairs = vec![];
                    for (key, val) in obj {
                        let mut pair = String::new();
                        let key_string = HeapManager::get_string(key);
                        pair = pair.add(&key_string);
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
}