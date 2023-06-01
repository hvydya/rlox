use std::rc::Rc;
use debug::disassemble_chunk;
use vm::VM;

use crate::{chunk::{Chunk}};

pub mod chunk;
pub mod debug;
pub mod value;
pub mod vm;

fn main() {
    // TODO figure out how to pass mutable chunk around with Rc in picture.
    // Use case is this:
    // We need to pass a mutable reference around for all funcs
    // Multiple vars will hold a mutable (or not) reference for the same chunk.
    //
    // As we build the interpreter we might not need the above requirements.
    let mut chunk = Chunk::init_chunk();
    let mut i = chunk.add_constant(10);
    chunk.write_chunk(chunk::OpCode::OpConstant.into(), 123);
    chunk.write_chunk(i as u8, 123);

    i = chunk.add_constant(20);
    chunk.write_chunk(chunk::OpCode::OpConstant.into(), 123);
    chunk.write_chunk(i as u8, 123);

    chunk.write_chunk(chunk::OpCode::OpAdd.into(), 123);

    i = chunk.add_constant(10);
    chunk.write_chunk(chunk::OpCode::OpConstant.into(), 123);
    chunk.write_chunk(i as u8, 123);

    chunk.write_chunk(chunk::OpCode::OpDivide.into(), 123);

    chunk.write_chunk(chunk::OpCode::OpReturn.into(), 123);
    // disassemble_chunk(&chunk, "test chunk");
    let rc = Rc::new(chunk);
    let mut vm = VM::init_vm();
    vm.interpret(&rc);
    // chunk.free_chunk();
}