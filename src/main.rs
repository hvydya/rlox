use crate::{chunk::{init_chunk, write_chunk, free_chunk}, debug::disassemble_chunk};

pub mod chunk;
pub mod debug;

fn main() {
    let mut x = init_chunk();
    write_chunk(&mut x, chunk::OpCode::OpReturn);
    disassemble_chunk(&x, "test chunk");
    free_chunk(&mut x);
}