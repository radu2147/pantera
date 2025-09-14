use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use clap::Parser;
use clap_derive::Parser;
use pantera_compiler::compiler::Compiler;
use pantera_heap::heap::HeapManager;
use pantera_parser::lexer::Lexer;
use pantera_parser::parser::Parser as PanteraParser;
use pantera_vm::gc::GC;
use pantera_vm::stack::Stack;
use pantera_vm::VM;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    file_name: Option<String>,
}

pub fn run_pantera() {
    let cli = Cli::parse();

    if let Some(name) = cli.file_name.as_deref() {
        if !name.ends_with(".pant") {
            panic!("Cannot compile a file with the wrong extension");
        }
        let mut file = match File::open(name) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't open {}: {}", name, why)
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", name, why),
            Ok(_) => {
                let lexer = Lexer::new(&s);
                let parser = PanteraParser::new(lexer.scan_tokens().unwrap());

                let mut heap_manager = HeapManager::new();

                let compiler = Compiler::new(&mut heap_manager);
                let code = compiler.compile(parser);
                let mut execution_stack = Stack::init();
                let mut globals = HashMap::new();
                let mut gc = GC {
                    heap_manager: &mut heap_manager,
                };
                let mut vm = VM::new(code, &mut execution_stack, &mut globals, &mut gc);
                vm.execute();
            }
        }
    }
}