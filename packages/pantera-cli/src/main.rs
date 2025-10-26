use std::fs::File;
use std::io::Read;
use clap::Parser;
use clap_derive::Parser;
use pantera_vm::{execute_with_options, Options};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    file_name: Option<String>,
    code: Option<String>,
    #[arg(short, long, default_value_t = 8)]
    max_heap_size: usize,
}

pub fn execute_cli(string: &str) {
    let max_heap_size = 10 * 1024;
    execute_cli_with_options(string, Options { max_heap_size });
}

pub fn execute_cli_with_options(string: &str, options: Options) {
    match execute_with_options(string, options) {
        Ok(string) => {
            println!("{}", string.join("\n"));
        },
        Err(err) => {
            println!("Pantera Error:");
            println!("{err}");
        }
    }
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

                execute_cli_with_options(&s, Options {max_heap_size});
            }
        }
    } else if let Some(code) = cli.code.as_deref() {
        execute_cli(&code);
    }
}

fn main() {
    run_pantera();
}