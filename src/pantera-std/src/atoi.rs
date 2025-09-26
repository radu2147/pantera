use std::cell::RefCell;
use std::rc::Rc;
use pantera_heap::heap::HeapManager;
use pantera_heap::stack::Stack;
use pantera_heap::value::Value;

pub fn atoi(stack: &mut Stack, _heap_manager: Rc<RefCell<HeapManager>>) {
    match stack.pop().unwrap() {
        Value::String(num_as_str) => {
            let number = HeapManager::get_string(num_as_str);
            match number.parse::<f32>() {
                Ok(num) => {
                    stack.push(Value::Number(num));
                },
                Err(_e) => {
                    panic!("Argument is not a stringified number")
                }
            }
        },
        _ => panic!("Argument is not a stringified number")
    }
}