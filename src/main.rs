use std::{env, fs};
use vm::{VM, InterpretResult};

pub mod chunk;
pub mod debug;
pub mod value;
pub mod vm;
pub mod compiler;
pub mod scanner;

fn repl() {
    println!("Not implemented!");
}

fn read_file(path : &str) -> String {
    let source_code = fs::read_to_string(path).expect("Couldn't read file!");
    source_code
}

fn run_file(path : &str, vm : &mut VM) {
    let source = read_file(path);
    let result : InterpretResult = vm.interpret_source(source);
    match result {
        InterpretResult::InterpretCompileError => panic!("Compile error"),
        InterpretResult::InterpretRuntimeError => panic!("Runtime error"),
        _ => (),
    }
}

fn main() {
    let mut vm = VM::init_vm();
    let args : Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl();
        return;
    }
    let arg = args.get(1);
    println!("{:?}", arg);
    run_file(arg.unwrap().as_str(), &mut vm);
    
}