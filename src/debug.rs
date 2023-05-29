use crate::chunk::{Chunk, OpCode};

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
    match op {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
    }
}

pub fn simple_instruction(name : &str, offset : usize) -> usize {
    println!("{name}");
    offset + 1
}