mod gc;
mod runtime_context;
pub mod vm;

use std::cell::RefCell;
use std::rc::Rc;
use pantera_compiler::compiler::Compiler;
use pantera_heap::heap::HeapManager;
use pantera_heap::stack::Stack;
use pantera_parser::lexer::Lexer;
use pantera_parser::parser::Parser;
use pantera_std::init_vm_globals;
use crate::gc::GC;
use crate::vm::VM;

pub struct Options {
    pub max_heap_size: usize
}

pub fn execute(string: &str) -> Result<Vec<String>, String> {
    let max_heap_size = 10 * 1024;
    execute_with_options(string, Options { max_heap_size })
}

pub fn execute_with_options(string: &str, options: Options) -> Result<Vec<String>, String> {
    let lexer = Lexer::new(&string);
    let parser = Parser::new(lexer.scan_tokens().unwrap());

    let heap_manager = Rc::new(RefCell::new(HeapManager::new(options.max_heap_size)));

    let compiler = Compiler::new(Rc::clone(&heap_manager));
    let code = compiler.compile(parser)?;
    let mut execution_stack = Stack::init();
    let mut globals = init_vm_globals();
    let mut gc = GC {
        heap_manager: Rc::clone(&heap_manager),
        max_heap_size: options.max_heap_size
    };
    let mut vm = VM::new(code, &mut execution_stack, &mut globals, &mut gc, Rc::clone(&heap_manager));
    vm.execute()
}