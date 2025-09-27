use std::cell::RefCell;
use std::rc::Rc;
use pantera_heap::array::Array;
use pantera_heap::heap::HeapManager;
use pantera_heap::stack::Stack;
use pantera_heap::value::Value;

pub fn internal_iterable_get(stack: &mut Stack, _heap_manager: Rc<RefCell<HeapManager>>) {
    let Value::Number(index) = stack.pop().unwrap() else {panic!("Expected number as second argument")};
    let collection = stack.pop().unwrap();
    match collection {
        Value::Array(ptr) => unsafe {
            let arr = Array::from(ptr);
            let Some(element) = arr.get(index as usize) else { panic!("List index {index} out of range") };

            stack.push(element);
        },
        Value::Object(_ptr) => {
            todo!()
        },
        _ => panic!("Type of object is not iterable")
    }
}