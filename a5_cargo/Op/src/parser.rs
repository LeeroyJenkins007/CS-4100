use crate::lexer::{LexerState};
use crate::lexer::Token::*;
use crate::types::*;
use crate::types::Exp::*;
use crate::types::Binop::*;
use crate::types::Unop::*;
use crate::types::Value::*;
use crate::types::Type::*;



macro_rules! parse_err {
    ($l:expr, $err:expr) => {
        Err(format!("{} at {}:{} in '{}'",
                    $err, $l.info.line_no, $l.info.col_no, $l.rest))
    };
}

pub fn parse(s: &str) -> Result<Exp, String> {
    let mut l = LexerState::new(s);
    parse_prog(&mut l)
}

fn parse_prog(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("prog: expected a token") {
        LEFT_PAREN | EXPSTART => {
            //println!("FUNLIST START");
            let funlst = parse_funlist(l)?;
            l.eat(EXPSTART);
            //println!("MAIN EXP START");
            let mexp = parse_exp(l)?;
            Ok(EProg(Box::new(Prog{funlist: funlst,
                mainexp: mexp})))
        },
        tok => parse_err!(l, format!("prog: unexpected token {:?}", tok))

    }
}

fn parse_funlist(l: &mut LexerState) -> Result<Vec<Fun>, String> {
    let mut funvec: Vec<Fun> = Vec::new();
    match l.peek().expect("funlist: expected a token") {
        LEFT_PAREN => {
            //println!("FUNLIST");
            funvec = parse_fun(l)?;
            if let LEFT_PAREN = l.peek().unwrap() {
                //println!("ADDITIONAL FUNCTIONS");
                let mut additional = parse_funlist(l)?;
                funvec.append(&mut additional);
            }
            Ok(funvec)
        },
        EXPSTART => {
            //println!("EXPSTART NEXT");
            Ok(funvec)
        },
        tok => parse_err!(l, format!("funlist: unexpected token {:?}", tok))
    }
}

fn parse_fun(l: &mut LexerState) -> Result<Vec<Fun>, String> {
    let mut funvec: Vec<Fun> = Vec::new();
    match l.peek().expect("fun: expected a token") {
        LEFT_PAREN => {
            //println!("FUNCTION START");
            l.eat(LEFT_PAREN);
            l.eat(FUNDECL);
            let nme = parse_id(l)?;
            //println!("PARAMS START");
            let prams = parse_paramlist(l)?;
            //println!("PARAMS END");
            l.eat(RETARROW);
            let rt = parse_type(l)?;
            let e = parse_exp(l)?;
            l.eat(RIGHT_PAREN);
            funvec.push(Fun{name: nme, params: prams, retty: rt, body: e});
           // println!("FUNCTION END");
            Ok(funvec)
        },
        tok => parse_err!(l, format!("fun: unexpected token {:?}", tok))
    }
}

fn parse_id(l: &mut LexerState) -> Result<String, String> {
    match l.peek().expect("id: expected a token") {
        ID(name) => {
            //println!("ID: NAME: {:?}", name.clone());
            let mut nme = name.clone();
            l.eat(ID(name));
            Ok(nme)
        },
        tok => parse_err!(l, format!("id: unexpected token {:?}", tok))
    }
}

fn parse_explist(l: &mut LexerState) -> Result<Vec<Exp>, String> {
    let mut expvec: Vec<Exp> = Vec::new();
    
    match l.peek().expect("explist: expected a token") {
        LEFT_PAREN | TT_VAL | FALSE_VAL | TRUE_VAL | I32_VAL(_) | ID(_) => {
            let mut current_exp = parse_exp(l)?;
            expvec.push(current_exp);
            while let LEFT_PAREN | TRUE_VAL | FALSE_VAL | TT_VAL | I32_VAL(_) | ID(_)= l.peek().unwrap() {
                current_exp = parse_exp(l)?;
                expvec.push(current_exp)
            }
            Ok(expvec)
        },
        tok => parse_err!(l, format!("explist: unexpected token {:?}", tok))
    }
    
}

fn parse_paramlist(l: &mut LexerState) -> Result<Vec<Param>, String> {
    let mut parvec: Vec<Param> = Vec::new();
    match l.peek().expect("paramlist: expected a token") {
        LEFT_PAREN => {
            //println!("PARAM");
            parvec = parse_param(l)?;
            while let LEFT_PAREN = l.peek().unwrap() {
                //println!("ADDITIONAL PARAM");
                let mut additional = parse_param(l)?;
                parvec.append(&mut additional);
            }
            Ok(parvec)
        },
        RETARROW => { Ok(parvec) },
        tok => parse_err!(l, format!("paramlist: unexpected token {:?}", tok))
    }
}

fn parse_param(l: &mut LexerState) -> Result<Vec<Param>, String> {
    let mut parvec: Vec<Param> = Vec::new();
    match l.peek().expect("param: expected a token") {
        LEFT_PAREN => {
            l.eat(LEFT_PAREN);
            let id = parse_id(l)?;
            let rt = parse_type(l)?;
            l.eat(RIGHT_PAREN);
            parvec.push(Param{var: id, retty: rt});
            Ok(parvec)
        },
        RETARROW => {Ok(parvec)},
        tok => parse_err!(l, format!("param: unexpected token {:?}", tok))
    }
}

fn parse_type(l: &mut LexerState) -> Result<Type, String> {
    match l.peek().expect("type: expected a token") {
        INT_TYPE => {
            l.eat(INT_TYPE);
            Ok(TyInt)
        },
        BOOL_TYPE => {
            l.eat(BOOL_TYPE);
            Ok(TyBool)
        },
        UNIT_TYPE => {
            l.eat(UNIT_TYPE);
            Ok(TyUnit)
        },
        LEFT_PAREN => {
            l.eat(LEFT_PAREN);
            l.eat(ARRAY_TYPE);
            let ty = parse_type(l)?;
            l.eat(RIGHT_PAREN);
            Ok(TyArray(Box::new(ty)))
        },
        tok => parse_err!(l, format!("type: unexpected token {:?}", tok))
    }
}

fn parse_exp(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("exp: expected a token") {
        LEFT_PAREN => {
            l.eat(LEFT_PAREN);
            match l.peek().expect("exp: expected a token") {
                PLUS | MINUS | TIMES | DIVISION | LT | EQ => {
                    let ret = parse_binop(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                NEG => {
                    let ret = parse_unop(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                SEQ => {
                    let ret = parse_seq(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                LET => {
                    let ret = parse_let(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                ALLOC => {
                    let ret = parse_alloc(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                GET => {
                    let ret = parse_get(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                SET => {
                    let ret = parse_set(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                COND => {
                    let ret = parse_cond(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                CALL => {
                    let ret = parse_call(l);
                    l.eat(RIGHT_PAREN);
                    ret
                },
                ID(v) => {
                    //This is a funptr
                    //println!("HELLO LOOK AT ME: {:?}", v);
                    let ptr = parse_id(l).unwrap();
                    //println!("MADE IT");
                    let explist = parse_explist(l)?;
                    //println!("MADE IT 2");
                    l.eat(RIGHT_PAREN);
                    Ok(ECall(Box::new(Callexp{funptr: EFunptr(Box::new(Funptrexp{id: ptr})), args: explist})))
                    
                },
                FUNPTR => {
                    let ret = parse_funptr(l);
                    l.eat(RIGHT_PAREN);
                    //Ok(EFunptr(Box::new(Funptrexp{id: name})))
                    ret
                },
                tok => parse_err!(l, format!("paren exp: unexpected token {:?}", tok))
            }
        },
        TT_VAL => {
            l.eat(TT_VAL);
            Ok(EUnit)
        },
        FALSE_VAL => {
            l.eat(FALSE_VAL);
            Ok(EBool(false))
        },
        TRUE_VAL => {
            l.eat(TRUE_VAL);
            Ok(EBool(true))
        },
        I32_VAL(i) => {
            l.eat(I32_VAL(i));
            Ok(EI32(i))
        },
        ID(v) => {
            let ret = parse_id(l).unwrap();
            Ok(EId(ret))
        },
        tok => parse_err!(l, format!("exp: unexpected token {:?}", tok))
    }
}

fn parse_funptr(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("funptr: expected a token") {
        FUNPTR => {
            l.eat(FUNPTR);
            let name = parse_id(l)?;
            Ok(EFunptr(Box::new(Funptrexp{id: name})))
        },
        tok => parse_err!(l, format!("funptr: unexpected token {:?}", tok))
    }
}

fn parse_binop(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("binop: expected a token") {
        PLUS => {
            l.eat(PLUS);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            Ok(EBinop(Box::new(Binexp{op: Plus, lhs: e2, rhs: e1})))
        },
        MINUS => {
            l.eat(MINUS);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            Ok(EBinop(Box::new(Binexp{op: Minus, lhs: e2, rhs: e1})))
        },
        TIMES => {
            l.eat(TIMES);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            Ok(EBinop(Box::new(Binexp{op: Times, lhs: e2, rhs: e1})))
        },
        DIVISION => {
            l.eat(DIVISION);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            Ok(EBinop(Box::new(Binexp{op: Divide, lhs: e2, rhs: e1})))
        },
        LT => {
            l.eat(LT);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            Ok(EBinop(Box::new(Binexp{op: Lt, lhs: e2, rhs: e1})))
        },
        EQ => {
            l.eat(EQ);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            Ok(EBinop(Box::new(Binexp{op: Eq, lhs: e2, rhs: e1})))
        },
        tok => parse_err!(l, format!("bionp: unexpected token {:?}", tok))
    }
}

fn parse_unop(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("unop: expected a token") {
        NEG => {
            l.eat(NEG);
            let e1 = parse_exp(l)?;
            Ok(EUnop(Box::new(Unexp{op: Neg, exp: e1})))
        },
        tok => parse_err!(l, format!("unop: unexpected token {:?}", tok))
    }
}

fn parse_seq(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("seq: expected a token") {
        SEQ => {
            l.eat(SEQ);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            Ok(ESeq(Box::new(Seqexp{exp1: e1, exp2: e2})))
        },
        tok => parse_err!(l, format!("seq: unexpected token {:?}", tok))
    }
}

fn parse_let(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("let: expected a token") {
        LET => {
            l.eat(LET);
            let v = parse_exp(l)?;
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            Ok(ELet(Box::new(Letexp{var: v, exp1: e1, exp2: e2})))
        },
        tok => parse_err!(l, format!("let: unexpected token {:?}", tok))
    }
}

fn parse_alloc(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("alloc: expected a token") {
        ALLOC => {
            l.eat(ALLOC);
            let size = parse_exp(l)?;
            let init = parse_exp(l)?;
            Ok(EAlloc(Box::new(Allocexp{esize: size, einit: init})))
        },
        tok => parse_err!(l, format!("alloc: unexpected token {:?}", tok))
    }
}

fn parse_set(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("set: expected a token") {
        SET => {
            l.eat(SET);
            let arr = parse_exp(l)?;
            let idx = parse_exp(l)?;
            let val = parse_exp(l)?;
            Ok(ESet(Box::new(Setexp{earr: arr, eidx: idx, e1: val})))
        },
        tok => parse_err!(l, format!("set: unexpected token {:?}", tok))
    }
}

fn parse_get(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("get: expected a token") {
        GET => {
            l.eat(GET);
            let arr = parse_exp(l)?;
            let idx = parse_exp(l)?;
            Ok(EGet(Box::new(Getexp{earr: arr, eidx: idx})))
        },
        tok => parse_err!(l, format!("get: unexpected token {:?}", tok))
    }
}



fn parse_cond(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("cond: expected a token") {
        COND => {
            l.eat(COND);
            let cond = parse_exp(l)?;
            let exp1 = parse_exp(l)?;
            let exp2 = parse_exp(l)?;
            Ok(ECond(Box::new(Condexp{econd: cond, e1: exp1, e2: exp2})))
        },
        tok => parse_err!(l, format!("cond: unexpected token {:?}", tok))
    }
}

fn parse_call(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("call: expected a token") {
        CALL => {
            //println!("CALL");
            l.eat(CALL);
            match l.peek().expect("call l2: expected a token") {
                LEFT_PAREN => {
                    //l.eat(LEFT_PAREN);
                    //l.eat(FUNPTR);
                    //let fun_name = parse_id(l)?;
                    //l.eat(RIGHT_PAREN);
                    let fun_name = parse_exp(l)?;
                    let mut vecexp: Vec<Exp> = Vec::new();
                    vecexp = parse_explist(l)?;
                    Ok(ECall(Box::new(Callexp{funptr: fun_name, args: vecexp})))
                },
                ID(nme) => {
                    let fun_name = parse_id(l)?;
                    let mut vecexp: Vec<Exp> = Vec::new();
                    vecexp = parse_explist(l)?;
                    Ok(ECall(Box::new(Callexp{funptr: EFunptr(Box::new(Funptrexp{id: fun_name})), args: vecexp})))
                },
                tok => parse_err!(l, format!("call l2: unexpected token {:?}", tok))
            }
            //let fun_name = parse_id(l)?;
            //let mut vecexp: Vec<Exp> = Vec::new();
            //vecexp = parse_explist(l)?;
            //Ok(ECall(Box::new(Callexp{funptr: fun_name, args: vecexp})))
        },
        tok => parse_err!(l, format!("call: unexpected token {:?}", tok))
    }
}


