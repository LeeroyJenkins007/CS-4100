use crate::types::*;
use crate::types::Binop::*;
use crate::types::Exp::*;
use crate::types::Instr::*;

pub fn compile(e: &Exp) -> Vec<Instr> {
    println!("compile: {:?}", e);
    match e {
        EProg(prog) => {
            //println!("prog: {:?}", prog);
           // let mut fnlist = compile(&prog.funlist);
            compile(&prog.mainexp)
        },
        EI32(i) => vec![II32(*i)],
        EBinop(b) => {
            //println!("compile binop {:?}", b);
            let mut is_lhs = compile(&b.lhs);
            let mut is_rhs = compile(&b.rhs);
            let mut is_op = 
                match b.op.clone() {
                    BPlus => vec![IPlus],
                    BTimes => vec![ITimes],
                    BMinus => vec![IMinus],
                    BDivide => vec![IDivide],
                    BLt => vec![ILt],
                    BEq => vec![IEq]
                };
            let mut is = vec![];
            is.append(&mut is_lhs);
            is.append(&mut is_rhs);
            is.append(&mut is_op);
            is
        },
        EUnop(u) => {
            let mut is_exp = compile(&u.exp);
            let mut is_op = 
                match u.op.clone() {
                    UNeg => vec![INeg]
                };
            let mut is = vec![];
            is.append(&mut is_exp);
            is.append(&mut is_op);
            is
        },
        EBool(b) => vec![IBool(*b)],
        ESeq(s) => {
            let mut is_exp1 = compile(&s.exp1);
            let mut is_exp2 = compile(&s.exp2);
            let mut is_op = vec![ISeq];
            let mut is = vec![];
            is.append(&mut is_exp1);
            is.append(&mut is_op);
            is.append(&mut is_exp2);
            is
        },
        _ => vec![ILt],
    }// match e
}// fun
