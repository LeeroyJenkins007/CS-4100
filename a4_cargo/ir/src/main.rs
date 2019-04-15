use std::env;
use std::collections::HashMap;
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

static mut labelGen: i32 = 0;

fn fresh_label() -> String {
    let mut new_label = "_L";
    let mut added_num: String;
    unsafe {
        labelGen = labelGen + 1;
        added_num = labelGen.to_string();
    }
    new_label.to_owned() + &added_num.to_string()
}

fn main() -> Result<(), String>{

    let file = env::args().last().expect("cargo run file");
    let buf = fs::read_to_string(&file).expect(&format!("main: couldn't read {}", file));

    let mut rho: HashMap<String, u32> = HashMap::new();
    let mut counter: u32 = 0;

    match parse(&buf) {
        Ok(e) => {
            println!("setframe 0\npush Lmain\ncall\nhalt\nLmain:");
            let instrs = compile(&e, &mut rho, &mut counter);
            for instr in instrs {
                match instr {
                    II32(i) => println!("push {}", i),
                    IPlus => println!("binary +"),
                    ITimes => println!("binary *"),
                    IMinus => println!("binary -"),
                    IDivide => println!("binary /"),
                    ILt => println!("binary <"),
                    ISeq => println!("pop"),
                    IUnit => println!("push undef"),
                    IBool(b) => println!("push {}", b),
                    IEq => println!("binary =="),
                    INeg => println!("unary neg"),
                    IStore(i) => println!("store {}", i),
                    IAlloc => println!("alloc"),
                    IVar(v) => println!("var {}", v),
                    ISet => println!("set"),
                    IGet => println!("get"),
                    ICondThen(then, els, end) => {
                        println!("push {}\nbranch\npush true\npush {}\nbranch\n{}:", then, els, then);
                    },
                    ICondElse(then, els, end) => {
                        println!("push true\npush {}\nbranch\n{}:", end, els);
                    },
                    ICondEnd(then, els, end) => {
                        println!("push true\npush {}\nbranch\n{}:", end, end);
                    },
                    _ => println!("other stuff")
                }
            }
            println!("ret");
/*            println!("RHO:");
            for key in rho.keys() {
                println!("{}", key);
            }
            println!("COUNTER TOTAL: {}", counter);
*/            Ok(())
        },
        Err(err) => Err(err)
    }

}
