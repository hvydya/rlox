use crate::value::{ValueArray, init_value_array, write_value_array, Value};

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1
        }
    } 
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            _ => panic!("no suitable opcode found for u8 : {value}")
        }
    }
}

pub struct Chunk {
    pub code : Vec<u8>,
    pub count : usize,
    pub constants : ValueArray,
}

pub fn init_chunk() -> Chunk {
    Chunk { code: Vec::with_capacity(10), count: 0, constants: init_value_array() }
}

pub fn write_chunk(c : &mut Chunk, code : u8) {
    c.code.push(code);
    c.count += 1;
}

pub fn free_chunk(c : &mut Chunk) {
    *c = init_chunk();
}

pub fn add_constant(c : &mut Chunk, value : Value) -> usize {
    write_value_array(&mut c.constants, value);
    c.constants.count - 1
}