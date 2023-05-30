use chunk::add_constant;

use crate::{chunk::{init_chunk, write_chunk, free_chunk}, debug::disassemble_chunk};

pub mod chunk;
pub mod debug;
pub mod value;

fn main() {
    let mut x = init_chunk();
    let i = add_constant(&mut x, 10);
    write_chunk(&mut x, chunk::OpCode::OpConstant.into(), 123);
    write_chunk(&mut x, i as u8, 123);
    write_chunk(&mut x, chunk::OpCode::OpReturn.into(), 123);
    disassemble_chunk(&x, "test chunk");
    free_chunk(&mut x);
}