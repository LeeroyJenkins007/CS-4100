use std::env;
use std::collections::HashMap;
use std::fs;
use crate::types::Val;
use crate::types::Instr;

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

    let mut rho: HashMap<String, u32> = HashMap::new();
    rho.insert("$$".to_string(), 0);
    //let mut counter: u32 = 0;
    let mut startingInstrs:Vec<Instr> = Vec::new();
    //startingInstrs.push(SetFrame(0));
    //startingInstrs.push(Push(Val::Vlabel("Lmain".to_string())));
    //startingInstrs.push(Call);
    //startingInstrs.push(Halt);
    //startingInstrs.push(Label("Lmain:".to_string()));

    match parse(&buf) {
        Ok(e) => {
            //println!("Expression is: {:?}", e);
            let mut remainingInstrs = compile(&e, &mut rho);
            startingInstrs.append(&mut remainingInstrs);
            //startingInstrs.push(Ret);
            for instr in startingInstrs {
                match instr {
                    Label(lbl) => {
                        println!("{}", lbl);
                    },
                    Push(val) => {
                        println!("push {}", val.to_string());
                    },
                    Pop => {
                        println!("pop");
                    },
                    Peek(u) => {
                        println!("peek {}", u);
                    },
                    Unary(unop) => {
                        println!("unary {}", unop.to_string());
                    },
                    Binary(binop) => {
                        println!("binary {}", binop.to_string());
                    },
                    Swap => {
                        println!("swap");
                    },
                    Alloc => {
                        println!("alloc");
                    },
                    Set => {
                        println!("set");
                    },
                    Get => {
                        println!("get");
                    },
                    Var(u) => {
                        println!("var {}", u);
                    },
                    Store(u) => {
                        println!("store {}", u);
                    },
                    SetFrame(u) => {
                        println!("setframe {}", u);
                    },
                    Call => {
                        println!("call");
                    },
                    Ret => {
                        println!("ret");
                    },
                    Branch => {
                        println!("branch");
                    },
                    Halt => {
                        println!("halt");
                    }
                }
            }
            Ok(())
        },
        Err(err) => Err(err)
    }

}
