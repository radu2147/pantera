use pantera_compiler::compiler::Compiler;
use pantera_parser::lexer::Lexer;
use pantera_parser::parser::Parser;
use pantera_vm::VM;

fn main() {
    let lexer = Lexer::new("var a = {mama: null, rtvs:123, ab: {ert: \"trewrue\"}}; print a;");
    let parser = Parser::new(lexer.scan_tokens().unwrap());
    let mut compiler = Compiler::new();
    compiler.compile(parser);
    let mut vm = VM::init(compiler);
    vm.execute();
}
