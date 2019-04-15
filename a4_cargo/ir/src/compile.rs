use std::collections::HashMap;
use crate::types::*;
use crate::types::Binop::*;
use crate::types::Exp::*;
use crate::types::Instr::*;

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

pub fn compile(e: &Exp, prho: &mut HashMap<String, u32>, counter: &mut u32) -> Vec<Instr> {
    //println!("compile: {:?}", e);
    //let mut rho: HashMap<String, u32> = HashMap::new();
    let mut rho = prho;
    let mut cnt = counter;
    let test = cnt.clone();
    //println!("COUNT: {}", test);

    match e {
        EProg(prog) => {
            //println!("prog: {:?}", prog);
           // let mut fnlist = compile(&prog.funlist);
           //cnt = &mut (*cnt + 1);
            compile(&prog.mainexp, &mut rho, &mut (*cnt + 1))
        },
        EI32(i) => {
            let mut test = cnt.clone();
            test = test + 1;
            cnt = &mut (*cnt + 1);
            //println!("COUNT EI32 {}", test);
            vec![II32(*i)]
        },
        EUnit => {
            cnt = &mut (*cnt + 1);
            vec![IUnit]
        },
        EBinop(b) => {
            //println!("compile binop {:?}", b);
            let mut is_lhs = compile(&b.lhs, &mut rho, &mut cnt);
            let mut is_rhs = compile(&b.rhs, &mut rho, &mut cnt);
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
            let mut is_exp = compile(&u.exp, &mut rho, &mut cnt);
            let mut is_op = 
                match u.op.clone() {
                    UNeg => vec![INeg]
                };
            let mut is = vec![];
            is.append(&mut is_exp);
            is.append(&mut is_op);
            is
        },
        EBool(b) => {
            cnt = &mut (*cnt + 1);
            vec![IBool(*b)]
        },
        ESeq(s) => {
            let mut is_exp1 = compile(&s.exp1, &mut rho, &mut cnt);
            let mut is_exp2 = compile(&s.exp2, &mut rho, &mut cnt);
            let mut is_op = vec![ISeq];
            let mut is = vec![];
            is.append(&mut is_exp1);
            is.append(&mut is_op);
            is.append(&mut is_exp2);
            is
        },
        ELet(l) => {
            //let mut is_var = compile(&l.var);
            //let mut loc = *rho.get(l);
            //let mut is_var = vec![IStore(loc)];
            //let mut clone_var = &l.var.clone();
            let mut is_var = compile(&l.var, &mut rho, &mut cnt);
            let mut is_exp1 = compile(&l.exp1, &mut rho, &mut cnt);
            let mut is_exp2 = compile(&l.exp2, &mut rho, &mut cnt);
            let mut is = vec![];
            is.push(IUnit);
            is.append(&mut is_exp1);
            //is var should be a store i
            is.append(&mut is_var);
            //is exp2 should have a var i
            is.append(&mut is_exp2);
            if let EVar(var) = &l.var.clone() {
                is.push(IStore(*rho.get(var).unwrap())); 
            }
            is
        },
        EFun(f) => {
            //let mut is_fname = compile(&f.name);
            //let mut is_fparams = compile(&f.params);
            //let mut is_retty = compile(&f.retty);
            let mut is_exp = compile(&f.exp, &mut rho, &mut cnt);
            let mut is = vec![];
            is.append(&mut is_exp);
            is
        },
        EAlloc(a) => {
            let mut is_size = compile(&a.esize, &mut rho, &mut cnt);
            let mut is_init = compile(&a.einit, &mut rho, &mut cnt);
            let mut is_op = vec![IAlloc];
            let mut is = vec![];
            is.append(&mut is_size);
            is.append(&mut is_init);
            is.append(&mut is_op);
            is
        },
        EVar(v) => {
            let mut is = vec![];
            let mut is_vec = vec![];
            if rho.contains_key(&v.clone()) {
                //println!("fresh Variable");
                is_vec.push(IVar(*rho.get(&v.clone()).unwrap()));
            } else {
                rho.insert(v.to_owned().clone(), *cnt);
                is_vec.push(IStore(*cnt));
            }
            is.append(&mut is_vec);
            is
        },
        EGet(g) => {   
            let mut is_arr = compile(&g.earr, &mut rho, &mut cnt);
            let mut is_idx = compile(&g.eidx, &mut rho, &mut cnt);
            let mut is = vec![];
            is.append(&mut is_arr);
            is.append(&mut is_idx);
            is.push(IGet);
            is
        },
        ESet(s) => {
            let mut is_arr = compile(&s.earr, &mut rho, &mut cnt);
            let mut is_idx = compile(&s.eidx, &mut rho, &mut cnt);
            let mut is_e = compile(&s.e1, &mut rho, &mut cnt);
            let mut is = vec![];
            is.append(&mut is_arr);
            is.append(&mut is_idx);
            is.append(&mut is_e);
            is.push(ISet);
            is
        },
        ECond(c) => {
            let mut is_cond = compile(&c.econd, &mut rho, &mut cnt);
            let mut is_exp1 = compile(&c.e1, &mut rho, &mut cnt);
            let mut is_exp2 = compile(&c.e2, &mut rho, &mut cnt);
            let mut _Lthen = fresh_label();
            let mut _Lelse = fresh_label();
            let mut _Lend = fresh_label();
            let mut is = vec![];
            is.append(&mut is_cond);
            is.push(ICondThen(_Lthen.clone(), _Lelse.clone(), _Lend.clone()));
            is.append(&mut is_exp1);
            is.push(ICondElse(_Lthen.clone(), _Lelse.clone(), _Lend.clone()));
            is.append(&mut is_exp2);
            is.push(ICondEnd(_Lthen.clone(), _Lelse.clone(), _Lend.clone()));
            is
        },
        _ => vec![ILt],
    }// match e
}// fun
