use std::io;
use byteorder::ByteOrder;
use std::slice::Iter;
use std::env;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use byteorder::{BigEndian};
use std::collections::HashMap;
use std::char;

const HEAP_SIZE: u32 = 1024;
//const STACK_SIZE: u32 = 1024;

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
            16 => {Instr::Spawn},
            20 => {Instr::Print},
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
    Spawn,
    Print,
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

//copy from [from_addr] to [from_addr + size_of_array], from [from_heap] over to [to_heap]
fn copy(size_of_array: i32, from_addr: &usize, from_heap: &Vec<Val>, to_heap: &mut Vec<Val>) {
    
    //got from index -> index + size_of_array
    for index in 0..size_of_array + 1 {
        let from_heap_val = from_heap.get(*from_addr + index as usize).unwrap().clone();
        to_heap.push(from_heap_val);
    }
}

fn collect_garbage(heap: &mut Vec<Val>, stack: &mut Vec<Val>, size: u32) {
    let mut to_space: Vec<Val> = Vec::new();
    let mut address_track: HashMap<usize, usize> = HashMap::new();
    let mut next: u32 = 0;
    let mut scan: u32 = 0;

    eprintln!("GC start: heap_size = {} values", heap.len());

    //FOR each root address in the stack
    for index in 0..stack.len() {
        let stack_val = stack.get(index).unwrap().clone();
        //If the value in the stack is an address, then proceed..
        if let Val::Vaddr(from_addr) = stack_val {
            //if the pointer to the from_heap has not been copied over yet, then update it
            if !address_track.contains_key(&from_addr){
                address_track.insert(from_addr, to_space.len());
                if let Val::Vsize(array_size) = heap.get(from_addr).unwrap().clone() {
                    copy(array_size, &from_addr, &heap, &mut to_space);
                    next = next + (array_size as u32);
                }else {
                    copy(1, &from_addr, &heap, &mut to_space);
                    next = next + 1;
                }
            }else {
            }
            //either way the address need to be updated to the new address on the to_heap
            stack.remove(index);
            stack.insert(index, Val::Vaddr(*address_track.get(&from_addr).unwrap()));
        }
        //Otherwise, do nothing at all, ONLY concerned with addresses in the stack.
    }

    //Time to scan through the to_heap and search for addresses
    while scan <= next && next > 0 {
        let to_heap_val = to_space.get(scan as usize).unwrap().clone();
        if let Val::Vaddr(to_addr) = to_heap_val {
            //check if the address it points to has already been copied, and if not copy over and
            //update address_tracker
            if !address_track.contains_key(&to_addr) {
                //It has not yet been copied over, so first copy then update teh address value
                address_track.insert(to_addr, to_space.len());
                if let Val::Vsize(from_array_size) = heap.get(to_addr).unwrap().clone() {
                    //The value it points to is the start of an array
                    copy(from_array_size, &to_addr, &heap, &mut to_space);
                    next = next + (from_array_size as u32);
                }else{
                    //The value it points to is not an array size
                    next = next + 1;
                }
            }
            
            to_space.remove(scan as usize);
            to_space.insert(scan as usize, Val::Vaddr(*address_track.get(&to_addr).unwrap()));

        }
        
        scan = scan + 1;
    }

    //Updates the stack to point to the new location of chunk that is now in the to_space
    heap.clear();
    heap.append(&mut to_space);
    
    eprintln!("GC end: heap_size = {} values", heap.len());
    
    if ((heap.len() as u32) + size) > HEAP_SIZE {
        panic!("GC: Heap extends beyond {}, by adding {} to {}", HEAP_SIZE, size, heap.len());
    }
}


fn instr(vector_of_states: &mut Vec<State>, program_size: u32, thread_number: usize) {

    let mut program_state  = &mut vector_of_states[thread_number];
    //while !program_state.halt {
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
                                _ => panic!("UNOP: Cannot perform negation not on a boolean"),
                            }},
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
                                        _ => panic!("ADD: Cannot perform addition on value not i32"),
                                    }},
                                _ => panic!("ADD: Cannot perform addition on value not i32"),
                            }},
                        Binop::Mul => {
                            let op1 = program_state.stack.pop().unwrap();
                            let op2 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vi32(o1) => {
                                    match op2 {
                                        Val::Vi32(o2) => {program_state.stack.push(Val::Vi32(o1 * o2))},
                                        _ => panic!("MUL: Cannot perform multiplication on value not i32"),
                                    }},
                                _ => panic!("MUL: Cannot perform multiplication on value not i32"),
                            }},
                        Binop::Sub => {
                            let op1 = program_state.stack.pop().unwrap();
                            let op2 = program_state.stack.pop().unwrap();
                            match op1 {
                                Val::Vi32(o1) => {
                                    match op2 {
                                        Val::Vi32(o2) => {program_state.stack.push(Val::Vi32(o1 - o2))},
                                        _ => panic!("SUB: Cannot perform subtraction on value not i32"),
                                    }},
                                _ => panic!("SUB: Cannot perform subtraction on value not i32"),
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
                                            }else {panic!("DIV: Cannot divide by zero!");}},
                                        _ => panic!("DIV: Cannot perform division on value not i32"),
                                    }},
                                _ => panic!("DIV: Cannot perform division on value not i32"),
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
                                        _ => panic!("LT: Cannot perform less than on values not i32"),
                                    }},
                                _ => panic!("LT: Cannot perform less than on values not i32"),
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
                                        _ => panic!("EQ: Cannot perform equality on values not i32"),
                                    }},
                                _ => panic!("EQ: Cannot perform equality on values not i32"),
                            }},
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
                        if ((program_state.heap.len() as u32) + (size as u32)) < HEAP_SIZE {
                            program_state.stack.push(Val::Vaddr(program_state.heap.len()));
                            program_state.heap.push(Val::Vsize(size));
                            for _i in 1..size + 1{
                                program_state.heap.push(unit.clone());
                            }
                        }else {
                            //reccomended to just pass the state and thread
                            collect_garbage(&mut program_state.heap, &mut program_state.stack, size as u32);
                            program_state.stack.push(Val::Vaddr(program_state.heap.len()));
                            program_state.heap.push(Val::Vsize(size));
                            for _i in 1..size + 1{
                                program_state.heap.push(unit.clone());
                            }
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
                                _ => panic!("SET: Not a valid address for heap"),
                            }},
                        _ => panic!("SET: Not a valid value for heap index"),
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
                                _ => panic!("GET: Not a valid address to GET value from heap"),
                            }},
                        _ => panic!("GET: Cannot GET value at non i32 index on heat"),
                    }},
//VAR
                Instr::Var(u) => {
                    if (program_state.fp + *u) < program_state.stack.len() as u32 {
                        let nvar = program_state.stack.get((program_state.fp + *u) as usize).unwrap();
                        program_state.stack.push(nvar.clone());
                    }else {
                        panic!("VAR: Var value is longer than the stack length!");
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
                        _ => panic!("CALL: Invalid location on Call operation"),
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
                                _ => panic!("RET: Cannot return fp to a non location"),
                        }},
                        _ => panic!("RET: Cannot return pc to a non location"),
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
                                    _ => panic!("BRANCH: Not a valid condition for branching"),
                                }
                            }else {
                                panic!("BRANCH: Not a valid address to branch to");
                            }},
                        _ => panic!("BRANCH: Not a value to branch to"),
                    }},
//HALT
                Instr::Halt => {
                    program_state.halt = true;
                    //println!("{:?}", program_state.stack.pop().unwrap());
                    },
//PRINT                
                Instr::Print => {
                    let val_to_be_print = program_state.stack.pop().unwrap();
                    match val_to_be_print {
                        Val::Vi32(int) => {
                            print!("{}", char::from_u32(int as u32).unwrap());},
                        _ => {panic!("PRINT: Cannot print out values that are not I32");},
                    }},
//SPAWN       
                Instr::Spawn => {
                    let closure_address = program_state.stack.pop().unwrap();
                    //println!("SPAWN");
                    let heap_copy = program_state.heap.clone();
                    let mut new_thread_stack: Vec<Val> = Vec::new();
                    let mut new_thread_program: Vec<Instr> = Vec::new();
                    let mut funptr_location: Val;
                    let funptr: u32;

                    if let Val::Vaddr(closure) = closure_address {
                        funptr_location = program_state.heap.get(closure + (1 as usize)).unwrap().clone();
                        if let Val::Vloc(location) = funptr_location {
                            funptr = location;
                            //println!("Location: {}", funptr);
                        }else {
                            panic!("SPAWN: closure address in stack does not point to a Vloc");
                        }
                        //println!("Just Making sure this works so far: {:?}", funptr);
                    }else {
                        panic!("SPAWN: Value in stack is not an address to the heap!");
                    }
                    
                    //Constructing the stack to give the new thread
                    new_thread_stack.push(closure_address.clone());
                    new_thread_stack.push(Val::Vunit);
                    //ret_fp
                    new_thread_stack.push(Val::Vloc(program_state.fp));
                    //ret_pc force a halt on "returning"
                    let halt_location: u32 = program_state.program.len() as u32- 1;
                    new_thread_stack.push(Val::Vloc(halt_location));

                    //Giving the new thread a copy of the instruction list
                    new_thread_program = program_state.program.clone();

                    let new_thread = State {halt: false, pc: funptr, fp: 0, stack: new_thread_stack, heap: heap_copy, program: new_thread_program};

                    vector_of_states.push(new_thread);
                    

                },
            }
        }else {panic!("MAIN: PC is greater than program length!");}
    //}

}

fn main() -> io::Result<()>{
    let mut file_content = Vec::new();
    let mut stack_instr: Vec<Instr> = Vec::new();
    let program_stack: Vec<Val> = Vec::new();
    let program_heap: Vec<Val> = Vec::new();
    let mut halted_threads: Vec<bool> = Vec::new();
    let mut program_halted: bool = false;

    let quantum: u32 = 3;
    let mut thread_states: Vec<State> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1])?;
    let mut file = BufReader::new(file);

        file.read_to_end(&mut file_content);
    let mut iterator = file_content.iter();
    let program_size: u32 = u32::from_bin(&mut iterator);
    file_content.iter().next();
    for _i in 1..program_size + 1 {
        stack_instr.push(Instr::from_bin(&mut iterator));
    }

    let main_program_state = State { halt: false, pc: 0, fp: 0, stack: program_stack, heap: program_heap, program: stack_instr};
   
    thread_states.push(main_program_state);


    //change to while gc state does not halt
    while !thread_states[0].halt{
        //for mut each_state in &mut thread_states {
        for index_of_thread in 0..thread_states.len() {
            for number_of_exec in 1..quantum + 1 {
                //If this specific thread state has halted, then no need to continue the loop
                if (!thread_states[index_of_thread].halt) {
                    //println!("Thread: {}, Iteration: {}", index_of_thread, number_of_exec);
                    /*
                    if halted_threads.len() < thread_states.len() {
                        for udex in 1..(thread_states.len() - halted_threads.len()) {
                            halted_threads.push(false); 
                        }
                    }*/


                    instr(&mut thread_states, program_size, index_of_thread);
                }else {
                    //halted_threads[index_of_thread] = true;
                    /*
                    for check in &halted_threads {
                        if *check {
                            program_halted = true;
                        }else {
                            program_halted = false;
                            break;
                        }
                    }
                    */
                    /*
                    if index_of_thread == 0 && thread_states.len() >{
                        
                    }else {
                        println!("{:?}", thread_states[index_of_thread].stack.pop().unwrap());
                    }*/
                    break;
                }
            }//Number of executions per thread....set to whatever quantum is
        }//Each_thread
    }//While-loop

    Ok(())
}
