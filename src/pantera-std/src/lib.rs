pub mod sleep;
pub mod len;
mod input;
mod atoi;
mod internal_iterate_get;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use pantera_heap::heap::HeapManager;
use pantera_heap::stack::Stack;
use pantera_heap::value::{FunctionValue, Value};
use crate::atoi::atoi;
use crate::internal_iterate_get::internal_iterable_get;
use crate::input::input;
use crate::len::len;
use crate::sleep::sleep;

pub fn init_vm_globals() -> HashMap<u16, Value> {
    let mut globals = HashMap::new();

    for (ind, g) in STD_LIB.iter().enumerate() {
        globals.insert(ind as u16, Value::Function(FunctionValue::Builtin(g.func)));
    }

    globals
}

pub fn init_compiler_globals() -> HashMap<String, u16> {
    let mut globals = HashMap::new();

    for (ind, g) in STD_LIB.iter().enumerate() {
        globals.insert(g.name.to_string(), ind as u16);
    }

    globals
}

struct StdLibEntry {
    name: &'static str,
    func: fn(&mut Stack, Rc<RefCell<HeapManager>>)
}

impl StdLibEntry {
    const fn new(name: &'static str, func: fn(&mut Stack, Rc<RefCell<HeapManager>>)) -> Self {
        Self {
            name, func
        }
    }
}

macro_rules! generate_std_lib {
    ($($func:ident),*) => {
        const STD_LIB: [StdLibEntry; generate_std_lib!(@count $($func),*)] = [
            $(
                StdLibEntry::new(stringify!($func), $func),
            )*
        ];
    };
    (@count $($t:tt),*) => {
        <[()]>::len(&[$(generate_std_lib!(@sub $t)),*])
    };

    (@sub $t:tt) => { () };
}

generate_std_lib!(len, sleep, input, atoi, internal_iterable_get);