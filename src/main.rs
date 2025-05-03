use pantera_compiler::compiler::Compiler;
use pantera_parser::lexer::Lexer;
use pantera_parser::parser::Parser;
use pantera_vm::VM;

fn main() {
    let lexer = Lexer::new("var a = \"Mama are mere\n\"; var b = \"Mama are mee\"; print a + b;");
    let parser = Parser::new(lexer.scan_tokens().unwrap());
    let mut compiler = Compiler::new();
    compiler.compile(parser);
    let mut vm = VM::init(compiler);
    vm.execute();
}
