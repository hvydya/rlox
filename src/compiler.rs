use std::rc::Rc;

use crate::scanner::{init_scanner, TokenType};

pub fn compile(source : Rc<String>) {
    let mut scanner = init_scanner(source);
    let line = 0;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            // TODO print with padding
            print!("{} ", token.line);
        } else {
            print!("| ");
        }
        println!("{:?} '{}'", token.token_type, token.value);
        match token.token_type {
            TokenType::TokenEof => break,
            _ => ()
        }
    }
}