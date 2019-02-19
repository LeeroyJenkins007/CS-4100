use byteorder::{BigEndian, WriteBytesExt};
use std::collections::HashMap;
use std::io;
use std::env;
use std::io::BufRead;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use regex::Regex;

pub trait ToBin {
    fn to_bin(self: &Self) -> Vec<u8>;
}

impl ToBin for u32 {
    fn to_bin(self: &Self) -> Vec<u8>{
        let mut u_32_byte = Vec::new();
        u_32_byte.write_u32::<BigEndian>(*self).unwrap();
        u_32_byte
    }
}

impl ToBin for i32 {
    fn to_bin(self: &Self) -> Vec<u8>{
        let mut i_32_byte = Vec::new();
        i_32_byte.write_i32::<BigEndian>(*self).unwrap();
        i_32_byte
    }
}

impl ToBin for Val {
    fn to_bin(self: &Self) -> Vec<u8>{
        let mut val_byte = Vec::new();
        match self {
            Val::Vunit => {val_byte = [0].to_vec();},
            Val::Vi32(i) => {val_byte = [1].to_vec();
                val_byte.append(&mut i32::to_bin(&i));},
            Val::Vbool(b) => {
                if *b {
                    val_byte = [2].to_vec();
                }else {
                    val_byte = [3].to_vec();
                }},
            Val::Vloc(u) => {val_byte = [4].to_vec();
                val_byte.append(&mut u32::to_bin(&u))},
            Val::Vundef => {val_byte = [5].to_vec();},
        }
        val_byte
    }
}

impl ToBin for Unop {
    fn to_bin(self: &Self) -> Vec<u8>{
        let mut unop_byte = Vec::new();
        match self {
            Unop::Neg => {unop_byte = [0].to_vec();},
        }
        unop_byte
    }
}

impl ToBin for Binop {
    fn to_bin(self: &Self) -> Vec<u8>{
        let mut binop_byte = Vec::new();
        match self {
            Binop::Add => {binop_byte = [0].to_vec();},
            Binop::Mul => {binop_byte = [1].to_vec();},
            Binop::Sub => {binop_byte = [2].to_vec();},
            Binop::Div => {binop_byte = [3].to_vec();},
            Binop::Lt => {binop_byte = [4].to_vec();},
            Binop::Eq => {binop_byte = [5].to_vec();},
        }
        binop_byte
    }
}

impl ToBin for Instr {
    fn to_bin(self: &Self) -> Vec<u8>{
        let mut instr_byte = Vec::new();
        match self {
            Instr::Push(v) => {
                instr_byte = [0].to_vec();
                instr_byte.append(&mut Val::to_bin(&v));},
            Instr::Pop => {instr_byte = [1].to_vec();},
            Instr::Peek(u) => {instr_byte = [2].to_vec();
                instr_byte.append(&mut u32::to_bin(&u));},
            Instr::Unary(unop) => {instr_byte = [3].to_vec();
                instr_byte.append(&mut Unop::to_bin(&unop));},
            Instr::Binary(binop) => {instr_byte = [4].to_vec();
                instr_byte.append(&mut Binop::to_bin(&binop));},
            Instr::Swap => {instr_byte = [5].to_vec();},
            Instr::Alloc => {instr_byte = [6].to_vec();},
            Instr::Set => {instr_byte = [7].to_vec();},
            Instr::Get => {instr_byte = [8].to_vec();},
            Instr::Var(u) => {instr_byte = [9].to_vec();
                instr_byte.append(&mut u32::to_bin(&u));},
            Instr::Store(u) => {instr_byte = [10].to_vec();
                instr_byte.append(&mut u32::to_bin(&u));},
            Instr::SetFrame(u) => {instr_byte = [11].to_vec();
                instr_byte.append(&mut u32::to_bin(&u));},
            Instr::Call => {instr_byte = [12].to_vec();},
            Instr::Ret => {instr_byte = [13].to_vec();},
            Instr::Branch => {instr_byte = [14].to_vec();},
            Instr::Halt => {instr_byte = [15].to_vec();},
        }
        instr_byte
    }
}

enum Val {
    Vunit,
    Vi32(i32),
    Vbool(bool),
    Vloc(u32),
    Vundef,
}

enum Instr {
    Push(Val),
    Pop,
    Peek(u32),
    Unary(Unop),
    Binary(Binop),
    Swap,
    Alloc,
    Set,
    Get,
    Var(u32),
    Store(u32),
    SetFrame(u32),
    Call,
    Ret,
    Branch,
    Halt,
}

enum Unop {
    Neg,
}

enum Binop {
    Add,
    Mul,
    Sub,
    Div,
    Lt,
    Eq,
}

fn push_eval(str: String, map: &mut HashMap<String, u32>) -> Instr{
    match &*str {
        "true" => Instr::Push(Val::Vbool(true)),
        "false" => Instr::Push(Val::Vbool(false)),
        "undef" => Instr::Push(Val::Vundef),
        "tt" => Instr::Push(Val::Vunit),
        other => {
            if str.parse::<i32>().is_ok() {
                Instr::Push(Val::Vi32(str.parse::<i32>().unwrap()))
            }
            else if map.contains_key(other){ 
                Instr::Push(Val::Vloc(*map.get(other).unwrap()))
            } else {
                println!("OTHER: {} elif {} length {}", other, map.contains_key(other), map.len());
                for (key, val) in map.iter(){
                    println!("{} {}", key, val);
                }
                panic!("Not a valid Push instruction");
            }
            },
    }
}

fn is_u32(str: String) -> u32 {
    if str.parse::<u32>().is_ok() {
        str.parse::<u32>().unwrap()
    } else {
        panic!("NOT A U32");
    }
}

fn is_uniary(str: String) -> Unop {
    match &*str {
        "neg" => Unop::Neg,
        _ => panic!("NOT A REAL UNOP"),
    }
}

fn is_binary(str: String) -> Binop {
        match &*str {
            "+" => Binop::Add,
            "*" => Binop::Mul,
            "-" => Binop::Sub,
            "/" => Binop::Div,
            "<" => Binop::Lt,
            "==" => Binop::Eq,
            _ => panic!("NOT A REAL BINOP"),
        }
}

fn check_label(str: &String) -> bool {
    let re = Regex::new(r"_*(?-i)L[[:alnum:]]+").unwrap();
    if str.contains(":") {
        if re.is_match(str) {
            true
        }else {
            false
        }
    }else {
        false
    }
}

fn strip(str: &mut String) -> String{
    let pos = str.len() - 1;
    let return_str = str[0..pos].to_string();
    return_str
}

fn main() -> io::Result<()>{
    let mut stack = Vec::new();
    let mut pc = 0;
    let mut symbol_table = HashMap::new();
    let mut bytes_to_write: Vec<u8> = Vec::new();

    let args: Vec<String> = env::args().collect();


    let mut fileName = args[1].clone();
    fileName.truncate(fileName.len() - 2);
    fileName.push_str(".o");
    let mut output = File::create(fileName)?;
    
    let file = File::open(&args[1])?;
    let file = BufReader::new(file);

    for line in file.lines() {
        let mut checker = line.unwrap();
        if check_label(&checker) {
            let slice = strip(&mut checker);
            if !symbol_table.contains_key(&slice){
                symbol_table.insert(slice, pc);
            }else{
                panic!("Multiple instances of the same label");
            }
        }else {pc = pc + 1;}
    }
    
    let file = File::open(&args[1])?;
    let file = BufReader::new(file);
    
    for line in file.lines() {

        let mut temp = line.unwrap();

        let mut iter = temp.split_whitespace();
        match &*iter.next().unwrap().to_string() {
                "push" => stack.push(push_eval(iter.next().unwrap().to_string(), &mut symbol_table)),
                "pop" => stack.push(Instr::Pop),
                "peek" => stack.push(Instr::Peek(is_u32(iter.next().unwrap().to_string()))),
                "unary" => stack.push(Instr::Unary(is_uniary(iter.next().unwrap().to_string()))),
                "binary" => stack.push(Instr::Binary(is_binary(iter.next().unwrap().to_string()))),
                "swap" => stack.push(Instr::Swap),
                "alloc" => stack.push(Instr::Alloc),
                "set" => stack.push(Instr::Set),
                "get" => stack.push(Instr::Get),
                "var" => stack.push(Instr::Var(is_u32(iter.next().unwrap().to_string()))),
                "store" => stack.push(Instr::Store(is_u32(iter.next().unwrap().to_string()))),
                "setframe" => stack.push(Instr::SetFrame(is_u32(iter.next().unwrap().to_string()))),
                "call" => stack.push(Instr::Call),
                "ret" => stack.push(Instr::Ret),
                "branch" => stack.push(Instr::Branch),
                "halt" => stack.push(Instr::Halt),
                other => {
                    let slice = strip(&mut other.to_string());
                      if symbol_table.contains_key(&slice) {
                        continue
                      }  else {
                        println!("its not a label its {}", other);
                        panic!("NOT A PROPER INSTRUCTION");
                      }
                    
                    },
            }
    }
    let mut iter = 0;
    let mut write_vector = Vec::new();

    for instr in &stack {
        let mut mission = Instr::to_bin(instr);
        iter = iter + 1;
        bytes_to_write.append(&mut mission);
    }
   
    write_vector.write_u32::<BigEndian>(iter).unwrap();
    write_vector.append(&mut bytes_to_write);
    output.write_all(&write_vector)?;
    Ok(())
}
