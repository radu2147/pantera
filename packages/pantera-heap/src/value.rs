use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::rc::Rc;
use crate::heap::{HeapManager, Ptr};
use crate::stack::Stack;

#[derive(Debug, Clone)]
pub enum FunctionValue {
    Builtin(fn(&mut Stack, Rc<RefCell<HeapManager>>)),
    UserDefined(usize, u8)
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Bool(bool),
    Null,
    Function(FunctionValue),
    String(Ptr),
    Object(Ptr),
    Array(Ptr)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => f.write_str(&num.to_string()),
            Self::Null => f.write_str("null"),
            Self::Bool(val) => f.write_str(&val.to_string()),
            Self::Function(_) => f.write_str("[function]"),
            Self::String(ptr) => {
                let str = HeapManager::get_string(*ptr);
                f.write_str(&str.to_string())
            },
            Self::Object(obj_ptr) => {
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
            },
            Self::Array(arr_ptr) => {
                let arr = HeapManager::get_array(*arr_ptr);
                let mut str = String::new();
                str = str.add("[ ");
                let mut pairs = vec![];
                for val in arr {
                    let mut pair = String::new();
                    pair = pair.add(&format!("{}", val));
                    pairs.push(pair);
                }
                str = str.add(pairs.join(", ").as_str());
                str = str.add(" ]");

                f.write_str(&str)
            }
        }
    }
}