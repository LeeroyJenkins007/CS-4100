use std::env;
use std::fs;

#[allow(dead_code)]
mod lexer;
use lexer::{LexerState, Token};

#[allow(dead_code)]
mod parser;
use parser::{parse};


#[allow(dead_code)]
mod types;
use crate::types::Instr::*;

#[allow(dead_code)]
mod compile;
use compile::{compile};



fn main() -> Result<(), String>{

    let file = env::args().last().expect("cargo run file");
    let buf = fs::read_to_string(&file).expect(&format!("main: couldn't read {}", file));

    match parse(&buf) {
        Ok(e) => {
            println!("setframe 0\npush Lmain\ncall\nhalt\nLmain:");
            let instrs = compile(&e);
            for instr in instrs {
                match instr {
                    II32(i) => println!("push {}", i),
                    IPlus => println!("binop +"),
                    ITimes => println!("binary *"),
                    IMinus => println!("binary -"),
                    IDivide => println!("binary /"),
                    ILt => println!("binary <"),
                    ISeq => println!("pop"),
                    IBool(b) => println!("push {}", b),
                    IEq => println!("binary =="),
                    INeg => println!("unary neg"),
                    _ => println!("other stuff")
                }
            }
            println!("ret");
            Ok(())
        },
        Err(err) => Err(err)
    }

}
