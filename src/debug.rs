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
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("    | ");
    } else {
        print!("{} ", chunk.lines[offset]);
    }
    let op = &chunk.code[offset];
    let opcode = OpCode::from(*op);
    match opcode {
        OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant => constant_instruction("OP_CONSTANT", chunk, offset),
        OpCode::OpNegate => simple_instruction("OP_NEGATE", offset),
        OpCode::OpAdd => simple_instruction("OP_ADD", offset),
        OpCode::OpSubtract => simple_instruction("OP_SUBTRACT", offset),
        OpCode::OpMultiply => simple_instruction("OP_MULTIPLY", offset),
        OpCode::OpDivide => simple_instruction("OP_DIVIDE", offset),
        // _ => panic!("Disassemble not implemented for {:?}", opcode)
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