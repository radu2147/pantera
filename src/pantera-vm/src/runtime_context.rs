use std::collections::HashMap;
use pantera_heap::value::Value;
use crate::stack::Stack;

pub struct RuntimeContext<'a> {
    pub execution_stack: &'a Stack,
    pub globals: &'a HashMap<u16, Value>,
}