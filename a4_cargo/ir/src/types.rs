use std::string::{ToString};


#[derive(Debug,Clone)]
pub enum Binop {
    BPlus,
    BTimes,
    BMinus,
    BDivide,
    BLt,
    BEq,
}


#[derive(Debug)]
pub struct Binexp {
    pub op: Binop,
    pub lhs: Exp,
    pub rhs: Exp
}


pub enum Value {
    Int(i32),
    True,
    False,
    Unit,
}


#[derive(Debug, Clone)]
pub enum Unop {
    UNeg,
}

#[derive(Debug)]
pub struct Unexp {
    pub op: Unop,
    pub exp: Exp
}

pub type Variable = String;

#[derive(Debug)]
pub struct Prog {
    pub funlist: Vec<Fun>,
    pub mainexp: Exp
}

#[derive(Debug)]
pub struct Fun {
    pub name: String,
    // name
    // params
    // retty
    // exp
}

#[derive(Debug)]
pub struct Letexp {
    pub var: Variable,
    pub exp1: Exp,
    pub exp2: Exp
}

#[derive(Debug)]
pub struct Seqexp {
    pub exp1: Exp,
    pub exp2: Exp
}

//use crate::types::Value::*;

pub struct Allocexp {
    pub esize: crate::types::Value,
    pub einit: Value
}

pub struct Setexp {
    pub earr: Variable,
    pub eidx: crate::types::Value,
    pub e1: Value,
}

pub struct Getexp {
    pub earr: Variable,
    pub eidx: crate::types::Value,
}

pub struct Condexp {
    pub cond: Exp,
    pub e1: Exp,
    pub e2: Exp,
}

pub struct Funptrexp {
    pub funptr: Variable,
}

#[derive(Debug)]
pub enum Exp {
    EI32(i32),
    EBool(bool),
    EProg(Box<Prog>),
    EFun(Box<Fun>),
    EBinop(Box<Binexp>),
    EUnop(Box<Unexp>),
    ESeq(Box<Seqexp>),
    ELet(Box<Letexp>),
}

use crate::types::Exp::*;

#[derive(Debug,Clone)]
pub enum Instr {
    IPlus,
    ITimes,
    IMinus,
    IDivide,
    ILt,
    IEq,
    ISeq,
    INeg,
    IBool(bool),
    II32(i32),
}

use crate::types::Instr::*;
