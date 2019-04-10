use std::env;
use std::fs;

mod lexer;
use lexer::{LexerState, Token};

fn main() {

    let file = env::args().last().expect("cargo run file");
    let buf = fs::read_to_string(&file).expect(&format!("main: couldn't read {}", file));
    println!("tokens are:");
    let mut l = LexerState::new(&buf);
    loop {
        if let Some(tok) = l.next() {
            println!("{:?}", tok);
        }else{ break; }
    }

    println!("Hello, world!");
}
