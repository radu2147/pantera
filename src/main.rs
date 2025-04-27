use pantera_compiler::compiler::Compiler;
use pantera_parser::lexer::Lexer;
use pantera_parser::parser::Parser;
use pantera_vm::VM;

fn main() {
    let lexer = Lexer::new("fun sum(a)with(b) {var c = a + b, rez = c; if c < 0 {rez = c^2;} return rez;} fun blk {loop 1..10 { print sum(it)with(-5); }} blk();");
    let parser = Parser::new(lexer.scan_tokens().unwrap());
    let mut compiler = Compiler::new();
    compiler.compile(parser);
    let mut vm = VM::init(compiler);
    vm.execute();
}
