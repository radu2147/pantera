use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use pantera_heap::value::Value;
use std::time::Duration;
use pantera_heap::heap::HeapManager;
use pantera_heap::stack::Stack;

pub fn sleep(stack: &mut Stack, _heap_manager: Rc<RefCell<HeapManager>>) {
    match stack.pop().unwrap() {
        Value::Number(num) => {
            thread::sleep(Duration::from_secs(num as u64));
            stack.push(Value::Null)
        },
        _ => panic!("Wrong argument to sleep function")
    }
}
