use std::rc::Rc;

use crate::{chunk::{Chunk, OpCode}, value::{Value, print_value}, debug::disassemble_instruction};

pub struct VM {
    pub chunk : Option<Rc<Chunk>>,
    pub ip : u8,
    pub stack : Vec<Value>,
    pub stk_top : usize
}

impl VM {

    fn push(&mut self, value : Value) {
        self.stack.push(value);
        self.stk_top += 1;
    }

    fn pop(&mut self) -> Value {
        if self.stk_top == 0 {
            panic!("Stack empty. But tried to pop!");
        }
        self.stk_top -= 1;
        self.stack.pop().unwrap()
    }

    pub fn init_vm() -> VM { VM { chunk: None, ip: 0, stack: Vec::with_capacity(256), stk_top: 0 } }

    fn reset_stack(&mut self) {
        self.stk_top = 0;
    }

    pub fn free_vm(&mut self) {
        self.reset_stack();
    }

    pub fn interpret(&mut self, chunk : &Rc<Chunk>) -> InterpretResult {
        self.chunk = Some(Rc::clone(chunk));
        self.ip = 0;
        return self.run();
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.ip - 1
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte();
        let chunk = self.chunk.as_ref().unwrap();
        chunk.constants.values[chunk.code[byte as usize] as usize]
    }

    fn binary_op(&mut self, op : OpCode) {
        let b = self.pop();
        let a = self.pop();
        let res = match op {
            OpCode::OpAdd => a + b,
            OpCode::OpSubtract => a - b,
            OpCode::OpMultiply => a * b,
            OpCode::OpDivide => a / b,
            _ => panic!("OpCode {:?} is not configured for binary operations!", op)
        };
        self.push(res);
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            // TODO do only if enabled
            // print!("            ");
            // for i in &self.stack {
            //     print!("[ ");
            //     print_value(*i);
            //     print!(" ]");
            // }
            // print!("\n");
            // disassemble_instruction(self.chunk.as_ref().unwrap(), self.ip as usize);
            // println!("{:?}", self.chunk.as_ref().unwrap().code);
            let instruction_ind : u8 = self.read_byte();
            let instruction = self.chunk.as_ref().unwrap().code[instruction_ind as usize];
            let op = OpCode::from(instruction);
            match op {
                OpCode::OpReturn => {
                    print_value(self.pop());
                    print!("\n");
                    break;
                },
                OpCode::OpConstant => {
                    let constant : Value = self.read_constant();
                    self.push(constant);
                    // print_value(constant);
                    // print!("\n");
                },
                OpCode::OpNegate => {
                    let val = self.pop();
                    self.push(-val);
                },
                OpCode::OpAdd => self.binary_op(op),
                OpCode::OpSubtract => self.binary_op(op),
                OpCode::OpMultiply => self.binary_op(op),
                OpCode::OpDivide => self.binary_op(op),
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


