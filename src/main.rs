use std::collections::HashMap;
use pantera_compiler::compiler::Compiler;
use pantera_heap::heap::HeapManager;
use pantera_parser::lexer::Lexer;
use pantera_parser::parser::Parser;
use pantera_vm::gc::GC;
use pantera_vm::stack::Stack;
use pantera_vm::VM;

fn main() {
    let lexer = Lexer::new("var a = \"2\" + \"22\" + \"23\"+ \"23\"+ \"23\"+ \"23\"+ \"23\"+ \"23\"+ \"23\" + \"52\" + \"62\" + \"22\" + \"32\" + \"32\"; print a;");
    let parser = Parser::new(lexer.scan_tokens().unwrap());

    let mut heap_manager = HeapManager::new();

    let mut compiler = Compiler::new(&mut heap_manager);
    let code = compiler.compile(parser);
    let mut execution_stack = Stack::init();
    let mut globals = HashMap::new();
    let mut gc = GC {
        heap_manager: &mut heap_manager,
    };
    let mut vm = VM::new(code, &mut execution_stack, &mut globals, &mut gc);
    vm.execute();
}
