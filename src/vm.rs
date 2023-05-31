use std::rc::Rc;

use crate::{chunk::{Chunk, OpCode}, value::{Value, print_value}, debug::disassemble_instruction};

pub struct VM {
    pub chunk : Option<Rc<Chunk>>,
    ip : u8,
}

impl VM {
    pub fn init_vm() -> VM { VM { chunk: None, ip: 0 } }
    pub fn free_vm(&mut self) {}

    pub fn interpret(&mut self, chunk : &Rc<Chunk>) -> InterpretResult {
        self.chunk = Some(Rc::clone(chunk));
        self.ip = 0;
        return self.run();
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.ip
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        let chunk = self.chunk.as_ref().unwrap();
        chunk.constants.values[chunk.code[byte as usize] as usize]
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            // TODO do only if enabled
            // disassemble_instruction(self.chunk.as_ref().unwrap(), self.ip as usize);
            let instruction_ind : u8 = self.ip;
            let instruction = self.chunk.as_ref().unwrap().code[instruction_ind as usize];
            let op = OpCode::from(instruction);
            match op {
                OpCode::OpReturn => break,
                OpCode::OpConstant => {
                    let constant : Value = self.read_constant();
                    print_value(constant);
                    print!("\n");
                }
            }
        }
        return InterpretResult::InterpretOk;
    }
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError
}


