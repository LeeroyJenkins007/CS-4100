use std::string::{ToString};
use std::collections::VecDeque;



type Address = usize;

#[derive(Debug,Clone)]
pub enum Binop {
    Plus,
    Times,
    Minus,
    Divide,
    Lt,
    Eq,
}

impl ToString for Binop {
    fn to_string(&self) -> String {
        match self {
            Binop::Times => "*".to_string(),
            Binop::Plus => "+".to_string(),
            Binop::Minus => "-".to_string(),
            Binop::Divide => "/".to_string(),
            Binop::Lt => "<".to_string(),
            Binop::Eq => "==".to_string()
        }
    }
}


#[derive(Debug, Clone)]
pub struct Binexp {
    pub op: Binop,
    pub lhs: Exp,
    pub rhs: Exp
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    True,
    False,
    Unit,
}


#[derive(Debug, Clone)]
pub enum Val {
    Vunit,
    Vi32(i32),
    Vbool(bool),
    Vloc(u32),
    Vundef,
    Vsize(i32),
    Vaddr(Address),
    Vlabel(String)
}

use crate::types::Val::*;

impl ToString for Val {
    fn to_string(&self) -> String {
        match self {
            Vunit => "tt".to_string(),
            Vi32(i) => i.to_string(),
            Vbool(b) => b.to_string(),
            Vloc(u) => u.to_string(),
            Vundef => "undef".to_string(),
            Vsize(i) => i.to_string(),
            Vaddr(a) => a.to_string(),
            Vlabel(l) => l.to_string()
        }
    }
}


#[derive(Debug, Clone)]
pub enum Unop {
    Neg,
}

impl ToString for Unop {
    fn to_string(&self) -> String {
        match self {
            Neg => "neg".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unexp {
    pub op: Unop,
    pub exp: Exp
}

pub type Variable = String;

#[derive(Debug, Clone)]
pub struct Prog {
    pub funlist: Vec<Fun>,
    pub mainexp: Exp
}

#[derive(Debug, Clone)]
pub enum Type {
    TyInt,
    TyBool,
    TyUnit,
    TyArray(Box<Type>)
}

#[derive(Debug, Clone)]
pub struct Param {
    pub var: String,
    pub retty: Type
}

#[derive(Debug, Clone)]
pub struct Fun {
    pub name: String,
    pub params: Vec<Param>,
    pub retty: Type,
    pub body: Exp
}

#[derive(Debug, Clone)]
pub struct Letexp {
    pub var: Exp,
    pub exp1: Exp,
    pub exp2: Exp
}

#[derive(Debug, Clone)]
pub struct Seqexp {
    pub exp1: Exp,
    pub exp2: Exp
}

//use crate::types::Value::*;

#[derive(Debug, Clone)]
pub struct Allocexp {
    pub esize: Exp,
    pub einit: Exp
}

#[derive(Debug, Clone)]
pub struct Setexp {
    pub earr: Exp,
    pub eidx: Exp,
    pub e1: Exp,
}

#[derive(Debug, Clone)]
pub struct Getexp {
    pub earr: Exp,
    pub eidx: Exp,
}

#[derive(Debug, Clone)]
pub struct Condexp {
    pub econd: Exp,
    pub e1: Exp,
    pub e2: Exp,
}

#[derive(Debug, Clone)]
pub struct Funptrexp {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Callexp {
    pub funptr: Exp,
    pub args: Vec<Exp>
}


#[derive(Debug, Clone)]
pub enum Exp {
    EI32(i32),
    EId(String),
    EUnit,
    EBool(bool),
    EProg(Box<Prog>),
    EBinop(Box<Binexp>),
    EUnop(Box<Unexp>),
    ESeq(Box<Seqexp>),
    ECond(Box<Condexp>),
    ELet(Box<Letexp>),
    ESet(Box<Setexp>),
    EGet(Box<Getexp>),
    ECall(Box<Callexp>),
    EFunptr(Box<Funptrexp>),
    EAlloc(Box<Allocexp>)
}

use crate::types::Exp::*;


#[derive(Debug, Clone)]
pub enum Instr {
    Label(String),
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
    Halt
}

use crate::types::Instr::*;
