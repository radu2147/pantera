use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use clap::Parser;
use clap_derive::Parser;
use pantera_compiler::compiler::Compiler;
use pantera_heap::heap::HeapManager;
use pantera_heap::stack::Stack;
use pantera_parser::lexer::Lexer;
use pantera_parser::parser::Parser as PanteraParser;
use pantera_std::init_vm_globals;
use pantera_vm::gc::GC;
use pantera_vm::VM;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    file_name: Option<String>,
    #[arg(short, long, default_value_t = 8)]
    max_heap_size: usize,
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
                let max_heap_size = cli.max_heap_size * 1024; // KB

                let lexer = Lexer::new(&s);
                let parser = PanteraParser::new(lexer.scan_tokens().unwrap());

                let heap_manager = Rc::new(RefCell::new(HeapManager::new(max_heap_size)));

                let compiler = Compiler::new(Rc::clone(&heap_manager));
                let code = compiler.compile(parser);
                let mut execution_stack = Stack::init();
                let mut globals = init_vm_globals();
                let mut gc = GC {
                    heap_manager: Rc::clone(&heap_manager),
                    max_heap_size
                };
                let mut vm = VM::new(code, &mut execution_stack, &mut globals, &mut gc, Rc::clone(&heap_manager));
                vm.execute();
            }
        }
    }
}