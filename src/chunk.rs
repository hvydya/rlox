#[derive(Debug)]
pub enum OpCode {
    OpReturn
}

pub struct Chunk {
    pub code : Vec<OpCode>,
    pub count : usize,
}

pub fn init_chunk() -> Chunk {
    Chunk { code: Vec::with_capacity(10), count: 0}
}

pub fn write_chunk(c : &mut Chunk, op : OpCode) {
    c.code.push(op);
    c.count += 1;
}

pub fn free_chunk(c : &mut Chunk) {
    *c = init_chunk();
}