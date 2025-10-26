use std::cell::RefCell;
use std::rc::Rc;
use text_io::read;
use pantera_heap::heap::HeapManager;
use pantera_heap::stack::Stack;
use pantera_heap::value::Value;

pub fn input(stack: &mut Stack, heap_manager: Rc<RefCell<HeapManager>>) {
    let line: String = read!("{}\n");
    let ptr = heap_manager.borrow_mut().allocate_string(line).unwrap();
    stack.push(Value::String(ptr));
}