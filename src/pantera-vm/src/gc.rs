use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use pantera_heap::heap::{HeapManager, Ptr};
use pantera_heap::stack::Stack;
use pantera_heap::value::Value;
use crate::runtime_context::RuntimeContext;

pub struct GC {
    pub heap_manager: Rc<RefCell<HeapManager>>,
}

impl GC {
    pub fn new(heap_manager: Rc<RefCell<HeapManager>>) -> Self {
        Self {
            heap_manager,
        }
    }
    fn mark(&self, context: &RuntimeContext) -> HashMap<Ptr, bool> {
        let mut iterable_objects = self.heap_manager.borrow().objects.clone();

        self.mark_globals(&mut iterable_objects, context.globals);
        self.mark_stack(&mut iterable_objects, context.execution_stack);

        iterable_objects
    }

    fn mark_strings(&self, context: &RuntimeContext) -> HashMap<Ptr, bool> {
        let mut iterable_strings = self.heap_manager.borrow().interned_strings.iter().filter(|(_str_ptr, is_compiled)| !**is_compiled).map(|(str_ptr, _is_compiled)| (*str_ptr, false)).collect::<HashMap<Ptr, bool>>();

        self.mark_globals(&mut iterable_strings, context.globals);
        self.mark_stack(&mut iterable_strings, context.execution_stack);

        iterable_strings
    }

    fn mark_globals(&self, objects: &mut HashMap<Ptr, bool>, globals: &HashMap<u16, Value>) {
        for (_key, val) in globals.iter() {
            self.mark_value(val, objects);
        }
    }

    fn mark_stack(&self, objects: &mut HashMap<Ptr, bool>, execution_stack: &Stack) {
        execution_stack.elements.iter().for_each(|val| self.mark_value(val, objects));
    }

    fn mark_value(&self, value: &Value, objects: &mut HashMap<Ptr, bool>) {
        match value {
            Value::String(ptr) => {
                objects.insert(*ptr, true);
            }
            Value::Object(ptr) => {
                let object = HeapManager::get_object(*ptr);
                object.values().for_each(|val| self.mark_value(val, objects));
                objects.insert(*ptr, true);
            }
            _ => {}
        }
    }

    fn sweep_objects(&mut self, objects: HashMap<Ptr, bool>) {
        for (key, value) in objects {
            if !value {
                self.heap_manager.borrow_mut().free_object(key);
            }
        }
    }

    fn sweep_strings(&mut self, strings: HashMap<Ptr, bool>) {
        for (key, value) in strings {
            if !value {
                self.heap_manager.borrow_mut().free_string(key);
            }
        }
    }

    pub fn collect(&mut self, context: &RuntimeContext) {
        if (self.heap_manager.borrow().objects.len() + self.heap_manager.borrow().interned_strings.len()) <= 10 {
            return;
        }

        let unmarked_objects = self.mark(context);
        let unmarked_strings = self.mark_strings(context);

        self.sweep_objects(unmarked_objects);
        self.sweep_strings(unmarked_strings);
    }
}
