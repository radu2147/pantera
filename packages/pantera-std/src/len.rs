use std::cell::RefCell;
use std::rc::Rc;
use pantera_heap::stack::Stack;
use pantera_heap::value::Value;
use pantera_heap::array::Array;
use pantera_heap::hash_table::HashTable;
use pantera_heap::heap::HeapManager;

pub fn len(stack: &mut Stack, _heap_manager: Rc<RefCell<HeapManager>>) {
    let collection = stack.pop().unwrap();
    match collection {
        Value::Array(arr) => unsafe {
            stack.push(Value::Number(Array::from(arr).get_count() as f32));
        },
        Value::Object(obj) => unsafe {
            stack.push(Value::Number(HashTable::from(obj).get_count() as f32));
        },
        _ => panic!("Object is not a collection to have a length")
    }
}