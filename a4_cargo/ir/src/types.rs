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
pub enum Unop {
    UNeg,
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
    TyInt(i32),
    TyBool(bool),
    TyUnit,
    TyArray
}

#[derive(Debug, Clone)]
pub struct Param {
    pub var: Exp,
    pub retty: Exp
}

#[derive(Debug, Clone)]
pub struct Fun {
    pub name: Exp,
    pub params: Vec<Param>,
    pub retty: Exp,
    pub exp: Exp
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
    pub funptr: Variable,
}

#[derive(Debug, Clone)]
pub enum Exp {
    EI32(i32),
    EVal(Value),
    EUnit,
    EVar(String),
    EBool(bool),
    EProg(Box<Prog>),
    EFun(Box<Fun>),
    EBinop(Box<Binexp>),
    EUnop(Box<Unexp>),
    ESeq(Box<Seqexp>),
    ECond(Box<Condexp>),
    ELet(Box<Letexp>),
    ESet(Box<Setexp>),
    EGet(Box<Getexp>),
    EAlloc(Box<Allocexp>)
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
    IStore(u32),
    IVar(u32),
    IAlloc,
    ISet,
    IGet,
    ILet,
    ISwap,
    ICondThen(String, String, String),
    ICondElse(String, String, String),
    ICondEnd(String, String, String),
    ISeq,
    INeg,
    IBool(bool),
    II32(i32),
    IUnit,
}

use crate::types::Instr::*;
