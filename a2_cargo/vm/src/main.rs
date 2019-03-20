use std::io;
use byteorder::ByteOrder;
use std::slice::Iter;
use std::env;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use byteorder::{BigEndian};


const HEAP_SIZE: u32 = 1024;
const STACK_SIZE: u32 = 1024;

pub trait FromBin {
        fn from_bin(iter: &mut Iter<u8>) -> Self;
}

impl FromBin for Instr {
        fn from_bin(iter: &mut Iter<u8>) -> Self{
        let slice = *iter.next().unwrap();
            match slice {
            0 => {Instr::Push(Val::from_bin(iter))},
            1 => {Instr::Pop},
            2 => {Instr::Peek(u32::from_bin(iter))},
            3 => {Instr::Unary(Unop::from_bin(iter))},
            4 => {Instr::Binary(Binop::from_bin(iter))},
            5 => {Instr::Swap},
            6 => {Instr::Alloc},
            7 => {Instr::Set},
            8 => {Instr::Get},
            9 => {Instr::Var(u32::from_bin(iter))},
            10 => {Instr::Store(u32::from_bin(iter))},
            11 => {Instr::SetFrame(u32::from_bin(iter))},
            12 => {Instr::Call},
            13 => {Instr::Ret},
            14 => {Instr::Branch},
            15 => {Instr::Halt},
            _ => panic!("Invalid Instruction"),
        }
    }
}

impl FromBin for Val {
        fn from_bin(iter: &mut Iter<u8>) -> Self{
            match *iter.next().unwrap() {
            0 => {Val::Vunit},
            1 => {Val::Vi32(i32::from_bin(iter))},
            2 => {Val::Vbool(true)},
            3 => {Val::Vbool(false)},
            4 => {Val::Vloc(u32::from_bin(iter))},
            5 => {Val::Vundef},
            _ => panic!("Invalid Value"),
        }
    }
}

impl FromBin for Unop {
        fn from_bin(iter: &mut Iter<u8>) -> Self{
        match *iter.next().unwrap() {
            0 => {Unop::Neg},
            _ => panic!("Invalid Unary operator"),
        }
    }
}

impl FromBin for Binop {
        fn from_bin(iter: &mut Iter<u8>) -> Self{
        match *iter.next().unwrap() {
            0 => {Binop::Add},
            1 => {Binop::Mul},
            2 => {Binop::Sub},
            3 => {Binop::Div},
            4 => {Binop::Lt},
            5 => {Binop::Eq},
            _ => panic!("Invalid Binary operator"),
        }
    }
}

impl FromBin for i32 {
    fn from_bin(iter: &mut Iter<u8>) -> Self{
        let mut i_32_byte = Vec::new();
        i_32_byte.push(*iter.next().unwrap());
        i_32_byte.push(*iter.next().unwrap());
        i_32_byte.push(*iter.next().unwrap());
        i_32_byte.push(*iter.next().unwrap());
        let byte_vector = BigEndian::read_i32(&i_32_byte);
        byte_vector
    }
}

impl FromBin for u32 {
        fn from_bin(iter: &mut Iter<u8>) -> Self{
        let mut u_32_byte: Vec<u8> = Vec::new();
        u_32_byte.push(*iter.next().unwrap());
        u_32_byte.push(*iter.next().unwrap());
        u_32_byte.push(*iter.next().unwrap());
        u_32_byte.push(*iter.next().unwrap());
        let byte_vector = BigEndian::read_u32(&u_32_byte);
        byte_vector
    }
}

type Address = usize;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Vunit,
    Vi32(i32),
    Vbool(bool),
    Vloc(u32),
    Vundef,

    Vsize(i32),
    Vaddr(Address),
}

#[derive(Debug, Clone)]
pub enum Instr {
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

#[derive(Debug, Clone)]
pub enum Unop {
    Neg,
}

#[derive(Debug, Clone)]
pub enum Binop {
    Add,
    Mul,
    Sub,
    Div,
    Lt,
    Eq,
}

#[derive(Debug, Clone)]
pub struct State {
    pub halt: bool,
    pub pc: u32,
    pub fp: u32,
    pub stack: Vec<Val>,
    pub heap: Vec<Val>,
    pub program: Vec<Instr>
}

fn main() -> io::Result<()>{
    let mut file_content = Vec::new();
    let mut stack_instr: Vec<Instr> = Vec::new();
    let mut program_stack: Vec<Val> = Vec::new();
    let mut program_heap: Vec<Val> = Vec::new();


    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])?;
    let mut file = BufReader::new(file);

        file.read_to_end(&mut file_content);
    let mut iterator = file_content.iter();
    let program_size: u32 = u32::from_bin(&mut iterator);
    file_content.iter().next();
    for i in 1..program_size + 1 {
        stack_instr.push(Instr::from_bin(&mut iterator));
    }

    let mut program_state = State { halt: false, pc: 0, fp: 0, stack: program_stack, heap: program_heap, program: stack_instr};

    while !program_state.halt {
        program_state.pc = program_state.pc + 1;
        if program_state.pc - 1 < program_size {
            match program_state.program.get((program_state.pc - 1) as usize).unwrap() {
//PUSH
                Instr::Push(val) => {program_state.stack.push(val.clone())},
//POP
                Instr::Pop => {program_state.stack.pop();},
//PEEK
                Instr::Peek(u) => {
                    let peek_val = program_state.stack.get(*u as usize).unwrap();
                    program_state.stack.push(peek_val.clone())},
//UNARY
                Instr::Unary(unop) => {
                    match unop {
                        Unop::Neg => {
                            let op1 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vbool(b) => {
                                        program_state.stack.push(Val::Vbool(!b));},
                                _ => panic!("Cannot perform negation not on a boolean"),
                            }},
                        _ => panic!("Not a calid Unary operation"),
                    }},
//BINARY
                Instr::Binary(binop) => {
                    match binop {
                        Binop::Add => {
                            let op1 = program_state.stack.pop().unwrap();
                            let op2 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vi32(o1) => {
                                    match op2 {
                                        Val::Vi32(o2) => {program_state.stack.push(Val::Vi32(o1 + o2))},
                                        _ => panic!("Cannot perform addition on value not i32"),
                                    }},
                                _ => panic!("Cannot perform addition on value not i32"),
                            }},
                        Binop::Mul => {
                            let op1 = program_state.stack.pop().unwrap();
                            let op2 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vi32(o1) => {
                                    match op2 {
                                        Val::Vi32(o2) => {program_state.stack.push(Val::Vi32(o1 * o2))},
                                        _ => panic!("Cannot perform multiplication on value not i32"),
                                    }},
                                _ => panic!("Cannot perform multiplication on value not i32"),
                            }},
                        Binop::Sub => {
                            let op1 = program_state.stack.pop().unwrap();
                            let op2 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vi32(o1) => {
                                    match op2 {
                                        Val::Vi32(o2) => {program_state.stack.push(Val::Vi32(o1 - o2))},
                                        _ => panic!("Cannot perform subtraction on value not i32"),
                                    }},
                                _ => panic!("Cannot perform subtraction on value not i32"),
                            }},
                        Binop::Div => {
                            let op1 = program_state.stack.pop().unwrap();
                            let op2 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vi32(o1) => {
                                    match op2 {
                                        Val::Vi32(o2) => {
                                            if o2 != 0 {
                                                program_state.stack.push(Val::Vi32(o1 / o2))
                                            }else {panic!("Cannot divide by zero!");}},
                                        _ => panic!("Cannot perform division on value not i32"),
                                    }},
                                _ => panic!("Cannot perform division on value not i32"),
                            }},
                        Binop::Lt => {
                            let op1 = program_state.stack.pop().unwrap();
                            let op2 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vi32(o1) => {
                                    match op2 {
                                        Val::Vi32(o2) => {
                                            if o1 < o2 {
                                                program_state.stack.push(Val::Vbool(true));
                                            }else {
                                                program_state.stack.push(Val::Vbool(false));
                                            }},
                                        _ => panic!("Cannot perform less than on values not i32"),
                                    }},
                                _ => panic!("Cannot perform less than on values not i32"),
                            }},
                        Binop::Eq => {
                            let op1 = program_state.stack.pop().unwrap();
                            let op2 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vi32(o1) => {
                                    match op2 {
                                        Val::Vi32(o2) => {
                                            if o1 == o2 {
                                                program_state.stack.push(Val::Vbool(true));
                                            }else {
                                                program_state.stack.push(Val::Vbool(false));
                                            }},
                                        _ => panic!("Cannot perform equality on values not i32"),
                                    }},
                                _ => panic!("Cannot perform equality on values not i32"),
                            }},
                        _ => {panic!("Not a valid Binary operation")},
                    }},
//SWAP
                Instr::Swap => {
                    let v1 = program_state.stack.pop().unwrap();
                    let v2 = program_state.stack.pop().unwrap();
                    program_state.stack.push(v1);
                    program_state.stack.push(v2);},
//ALLOC
                Instr::Alloc => {
                    let unit = program_state.stack.pop().unwrap();
                    let heap_size = program_state.stack.pop().unwrap();
                    if let Val::Vi32(size) = heap_size {
                        program_state.stack.push(Val::Vaddr(program_state.heap.len()));
                        if ((program_state.heap.len() as u32) + (size as u32)) < HEAP_SIZE {
                            program_state.heap.push(Val::Vsize(size));
                            for i in 1..size + 1{
                                program_state.heap.push(unit.clone());
                            }
                        }else {
                            panic!("Alloc expands Heap size beyond {}", HEAP_SIZE);
                        }
                    }
                },
//SET
                Instr::Set => {
                    let heap_val = program_state.stack.pop().unwrap();
                    let val_idx = program_state.stack.pop().unwrap();
                    let val_base = program_state.stack.pop().unwrap();
                    match val_idx {
                        Val::Vi32(idx) => {
                            match val_base {
                                Val::Vaddr(base) => {
                                    if (base as i32) + idx < program_state.heap.len() as i32 {
                                        program_state.heap.remove(base + (idx as usize) + 1);
                                        program_state.heap.insert(base + (idx as usize) + 1, heap_val.clone());
                                    }else{
                                        panic!("SET: base + idx larger than heap");
                                    }},
                                _ => panic!("Not a valid address for heap"),
                            }},
                        _ => panic!("Not a valid value for heap index"),
                    }},
//GET
                Instr::Get => {
                    let val_idx = program_state.stack.pop().unwrap();
                    let val_base = program_state.stack.pop().unwrap();
                    match val_idx {
                        Val::Vi32(idx) => {
                            match val_base {
                                Val::Vaddr(base) => {
                                    if base + (idx as usize) < program_state.heap.len() {
                                        let heap_val = program_state.heap.get(base + (idx as usize) + 1).unwrap();
                                        program_state.stack.push(heap_val.clone());
                                    }else{
                                        panic!("GET: base + idx is larger than the heap");
                                    }},
                                _ => panic!("Not a valid address to GET value from heap"),
                            }},
                        _ => panic!("Cannot GET value at non i32 index on heat"),
                    }},
//VAR
                Instr::Var(u) => {
                    if (program_state.fp + *u) < program_state.stack.len() as u32 {
                        let nvar = program_state.stack.get((program_state.fp + *u) as usize).unwrap();
                        program_state.stack.push(nvar.clone());
                    }else {
                        panic!("Var value is longer than the stack length!");
                    }},
//STORE
                Instr::Store(u) => {
                    let vnew = program_state.stack.pop().unwrap();
                    let index = (program_state.fp + u) as usize;
                    if (index as u32) <= (program_state.stack.len() as u32){
                        program_state.stack.remove(index);
                        program_state.stack.insert(index, vnew.clone());
                    }else {
                        panic!("STORE: index larger than size of stack!");
                    }},
//SETFRAME
                Instr::SetFrame(u) => {
                    program_state.stack.push(Val::Vloc(program_state.fp));
                    program_state.fp = ((program_state.stack.len() - (*u as usize) - 1) as u32);
                    },
//CALL
                Instr::Call => {
                    let target_addr = program_state.stack.pop().unwrap();
                    program_state.stack.push(Val::Vloc(program_state.pc));
                    match target_addr {
                        Val::Vloc(u) => program_state.pc = u,
                        _ => panic!("Invalid location on Call operation"),
                    }},
//RET
                Instr::Ret => {
                    let vret = program_state.stack.pop().unwrap();
                    let caller_pc = program_state.stack.pop().unwrap();
                    let caller_fp = program_state.stack.pop().unwrap();
                    match caller_pc{
                        Val::Vloc(pc) => {
                            match caller_fp {
                                Val::Vloc(fp) => {
                                    program_state.stack.truncate((program_state.fp) as usize);
                                    program_state.fp = fp;
                                    program_state.pc = pc;
                                    program_state.stack.push(vret);},
                                _ => panic!("Cannot return fp to a non location"),
                        }},
                        _ => panic!("Cannot return pc to a non location"),
                    }},
//BRANCH
                Instr::Branch => {
                    let target_location = program_state.stack.pop().unwrap();
                    let condition = program_state.stack.pop().unwrap();
                    match target_location {
                        Val::Vloc(loc) => {
                            if loc < (program_state.program.len() as u32){
                                match condition {
                                    Val::Vbool(b) => {
                                    if b {
                                        program_state.pc = loc;
                                    }},
                                    _ => panic!("Not a valid condition for branching"),
                                }
                            }else {
                                panic!("Not a valid address to branch to");
                            }},
                        _ => panic!("Not a value to branch to"),
                    }},
//HALT
                Instr::Halt => {
                    program_state.halt = true;
                    println!("{:?}", program_state.stack.pop().unwrap());},
            }
        }else {panic!("PC is greater than program length!");}
    }

    Ok(())
}
