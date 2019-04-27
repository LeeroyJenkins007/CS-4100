use std::collections::HashMap;
use crate::types::*;
use crate::types::Binop::*;
use crate::types::Exp::*;
use crate::types::Instr::*;
use crate::types::Unop::*;
use crate::types::Val::*;

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

fn incr_loc(rho: &mut HashMap<String, u32>) {
    if let Some(inc) = rho.get_mut("$$") {
        *inc = *inc + 1;
    }
}

fn decr_loc(rho: &mut HashMap<String, u32>) {
    if let Some(dec) = rho.get_mut("$$") {
        *dec = *dec - 1;
    }
}

pub fn compile_funlist(funlist: &Vec<Fun>, prho: &mut HashMap<String, u32>) -> Vec<Instr> {
    let mut rho = prho;
    let mut ret: Vec<Instr> = Vec::new();
    for fun in funlist {
        ret.append(&mut compile_fun(&fun, &mut rho));
    }
    ret
}

pub fn compile_fun(fun: &Fun, prho: &mut HashMap<String, u32>) -> Vec<Instr> {
    let mut rho = prho;
    let mut ret: Vec<Instr> = Vec::new();
    let mut counter = 0;
   
    for param in &fun.params {
        if !rho.contains_key(&param.var.clone()) {
            rho.insert(param.var.to_owned().clone(), counter);
            counter = counter + 1;
        }

    }

    let mut funlabel = "L".to_string() + &fun.name + ":";
    let mut is_body = compile(&fun.body, &mut rho);
    ret.push(Label(funlabel));
    ret.append(&mut is_body);
    ret
}

pub fn compile_explist(explist: &Vec<Exp>, prho: &mut HashMap<String, u32>) -> Vec<Instr> {
    let mut rho = prho;
    let mut ret: Vec<Instr> = Vec::new();
    for exp in explist {
        ret.append(&mut compile(&exp, &mut rho));
    }
    ret
}

pub fn compile(e: &Exp, prho: &mut HashMap<String, u32>) -> Vec<Instr> {
    let mut rho = prho;

    match e {
///Prog
        EProg(prog) => {
            let mut ret: Vec<Instr> = Vec::new();
            ret.push(SetFrame(0));
            ret.push(Push(Val::Vlabel("Lmain".to_string())));
            incr_loc(&mut rho);
            ret.push(Call);
            ret.push(Halt);
            ret.push(Label("Lmain:".to_string()));
            let mut mexp = compile(&prog.mainexp, &mut rho);
            ret.append(&mut mexp);
            let mut fnlist = compile_funlist(&prog.funlist, &mut rho);
            ret.append(&mut fnlist);
            ret.push(Ret);
            ret
        },
///i32
        EI32(i) => {
            incr_loc(&mut rho);
            vec![Push(Vi32(*i))]
        },
///Unit
        EUnit => {
            incr_loc(&mut rho);
            vec![Push(Vunit)]
        },
///Binop
        EBinop(b) => {
            decr_loc(&mut rho);
            let mut is_lhs = compile(&b.lhs, &mut rho);
            let mut is_rhs = compile(&b.rhs, &mut rho);
            let mut is_op = 
                match b.op.clone() {
                    Plus => vec![Binary(Plus)],
                    Times => vec![Binary(Times)],
                    Minus => vec![Binary(Minus)],
                    Divide => vec![Binary(Divide)],
                    Lt => vec![Binary(Lt)],
                    Eq => vec![Binary(Eq)]
                };
            let mut is = vec![];
            is.append(&mut is_lhs);
            is.append(&mut is_rhs);
            is.append(&mut is_op);
            is
        },
///Unop
        EUnop(u) => {
            let mut is_exp = compile(&u.exp, &mut rho);
            let mut is_op = 
                match u.op.clone() {
                    UNeg => vec![Unary(Neg)]
                };
            let mut is = vec![];
            is.append(&mut is_exp);
            is.append(&mut is_op);
            is
        },
///Bool
        EBool(b) => {
            incr_loc(&mut rho);
            vec![Push(Vbool(*b))]
        },
///Seq
        ESeq(s) => {
            let mut is_exp1 = compile(&s.exp1, &mut rho);
            let mut is_exp2 = compile(&s.exp2, &mut rho);
            let mut is_op = vec![Pop];
            let mut is = vec![];
            is.append(&mut is_exp1);
            is.append(&mut is_op);
            is.append(&mut is_exp2);
            is
        },
///Let
        ELet(l) => {
            incr_loc(&mut rho);
            let mut is_var = compile(&l.var, &mut rho);
            let mut is_exp1 = compile(&l.exp1, &mut rho);
            let mut is_exp2 = compile(&l.exp2, &mut rho);
            let mut is = vec![];
            is.push(Push(Vundef));
            is.append(&mut is_exp1);
            is.append(&mut is_var);
            is.append(&mut is_exp2);
            if let EId(var) = &l.var.clone() {
                is.push(Store(*rho.get(var).unwrap())); 
            }
            is
        },
///Call
        ECall(c) => {
            let mut vecexp = compile_explist(&c.args, &mut rho);
            let mut funp = compile(&c.funptr, &mut rho);
            let mut is = vec![];
            is.append(&mut vecexp);
            is.append(&mut funp);
            is
        },
///Funptr
        EFunptr(f) => {
            let mut setframe = *rho.get("$$").unwrap();
            let mut funLabel = "L".to_string() + &f.id;
            let mut is = vec![];
            is.push(Push(Vlabel(funLabel)));
            is.push(SetFrame(setframe));
            is.push(Swap);
            is.push(Call);
            is.push(Ret);
            is
        },
///Alloc
        EAlloc(a) => {
            decr_loc(&mut rho);
            let mut is_size = compile(&a.esize, &mut rho);
            let mut is_init = compile(&a.einit, &mut rho);
            let mut is_op = vec![Alloc];
            let mut is = vec![];
            is.append(&mut is_size);
            is.append(&mut is_init);
            is.append(&mut is_op);
            is
        },
///Id
        EId(v) => {
            let mut is = vec![];
            let mut is_vec = vec![];
            if rho.contains_key(&v.clone()) {
                is_vec.push(Var(*rho.get(&v.clone()).unwrap()));
            } else {
                let mut var_val = rho.get_mut("$$").unwrap().clone();
                rho.insert(v.to_owned().clone(),var_val);
                is_vec.push(Store(var_val));
            }
            is.append(&mut is_vec);
            is
        },
///Get
        EGet(g) => {   
            let mut is_arr = compile(&g.earr, &mut rho);
            let mut is_idx = compile(&g.eidx, &mut rho);
            let mut is = vec![];
            is.append(&mut is_arr);
            is.append(&mut is_idx);
            is.push(Get);
            is
        },
///Set
        ESet(s) => {
            let mut is_arr = compile(&s.earr, &mut rho);
            let mut is_idx = compile(&s.eidx, &mut rho);
            let mut is_e = compile(&s.e1, &mut rho);
            let mut is = vec![];
            is.append(&mut is_arr);
            is.append(&mut is_idx);
            is.append(&mut is_e);
            is.push(Set);
            is.push(Push(Vunit));
            is
        },
///Cond
        ECond(c) => {
            let mut is_cond = compile(&c.econd, &mut rho);
            let mut is_exp1 = compile(&c.e1, &mut rho);
            let mut is_exp2 = compile(&c.e2, &mut rho);
            let mut _Lthen = fresh_label();
            let mut _Lend = fresh_label();
            let mut is = vec![];
            is.append(&mut is_cond);
            is.push(Push(Vlabel(_Lthen.clone())));
            is.push(Branch);
            is.append(&mut is_exp2);
            is.push(Push(Vbool(true)));
            is.push(Push(Vlabel(_Lend.clone())));
            is.push(Branch);
            is.push(Label(_Lthen.clone() + ":"));
            is.append(&mut is_exp1);
            is.push(Push(Vbool(true)));
            is.push(Push(Vlabel(_Lend.clone())));
            is.push(Branch);
            is.push(Label(_Lend.clone() + ":"));
            is
        },
    }// match e
}// fun
