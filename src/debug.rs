use crate::{chunk::{Chunk, OpCode}, value::{print_value}};

pub fn disassemble_chunk(chunk : &Chunk, name : &str) {
    println!("== {name} ==");
    let mut offset : usize = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk : &Chunk, offset : usize) -> usize {
    print!("{offset} ");
    let op = &chunk.code[offset];
    let opcode = OpCode::from(*op);
    match opcode {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant => constant_instruction("OP_CONSTANT", chunk, offset)
    }
}

pub fn constant_instruction(name : &str, chunk : &Chunk, offset : usize) -> usize {
    let val : u8 = chunk.code[offset + 1];
    print!("{name} {val}");
    print_value(chunk.constants.values[val as usize]);
    println!("");
    offset + 2
}

pub fn simple_instruction(name : &str, offset : usize) -> usize {
    println!("{name}");
    offset + 1
}