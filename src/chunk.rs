use crate::value::{ValueArray, init_value_array, write_value_array, Value};

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide
}

pub enum BinaryOp {
    Plus,
    Minus,
    Asterisk,
    ForwardSlash
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate => 2,
            OpCode::OpAdd => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiply => 5,
            OpCode::OpDivide => 6
        }
    } 
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OpNegate,
            3 => OpCode::OpAdd,
            4 => OpCode::OpSubtract,
            5 => OpCode::OpMultiply,
            6 => OpCode::OpDivide,
            _ => panic!("no suitable opcode found for u8 : {value}")
        }
    }
}

pub struct Chunk {
    pub code : Vec<u8>,
    pub count : usize,
    pub constants : ValueArray,
    pub lines : Vec<usize>
}

impl Chunk {
    pub fn init_chunk() -> Chunk {
        Chunk { code: Vec::with_capacity(10), count: 0, constants: init_value_array(), lines: Vec::new() }
    }

    pub fn write_chunk(&mut self, code : u8, line : usize) {
        self.code.push(code);
        self.lines.push(line);
        self.count += 1;
    }

    pub fn free_chunk(&mut self) {
        *self = Chunk::init_chunk();
    }

    pub fn add_constant(&mut self, value : Value) -> usize {
        write_value_array(&mut self.constants, value);
        self.constants.count - 1
    }
}