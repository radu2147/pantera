use std::collections::HashMap;
use pantera_heap::stack::Stack;
use pantera_heap::value::Value;

pub struct RuntimeContext<'a> {
    pub execution_stack: &'a mut Stack,
    pub globals: &'a mut HashMap<u16, Value>,
}