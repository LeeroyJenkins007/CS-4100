//This is the parser set of functions and types
use crate::lexer::{LexerState};
use crate::lexer::Token::*;
use crate::types::*;
use crate::types::Exp::*;
use crate::types::Binop::*;
use crate::types::Unop::*;




macro_rules! parse_err {
    ($l:expr, $err:expr) => {
        Err(format!("{} at {}:{} in '{}'",
                    $err, $l.info.line_no, $l.info.col_no, $l.rest))
    };
}



fn parse_prog(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("prog: expected a token") {
        LEFT_PAREN | EXPSTART => {
           // println!("parsing funlist");
            let funlst = parse_funlist(l)?;
           // println!("funlist: {:?}", funlst);
            l.eat(EXPSTART);
           // println!("parsing exp");
            let mexp = parse_exp(l)?;
            println!("exp: {:?}", mexp);
            Ok(EProg(Box::new(Prog{funlist: funlst,
                mainexp: mexp})))
        },
        tok => parse_err!(l, format!("prog: unexpected token {:?}", tok))

    }
}

fn parse_funlist(l: &mut LexerState) -> Result<Vec<Fun>, String> {
    let funvec: Vec<Fun> = Vec::new();
    match l.peek().expect("funlist: expected a token") {
        LEFT_PAREN => {Ok(funvec)},
        EXPSTART => {Ok(funvec)},
        tok => parse_err!(l, format!("funlist: unexpected token {:?}", tok))
    }
    //Ok(EI32(17))
//    let funvec: Vec<Fun> = Vec::new();
//    Ok(funvec)
}

fn parse_exp(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("exp: expected a token") {
        LEFT_PAREN => {
            l.eat(LEFT_PAREN);
            match l.peek().expect("exp: expected a token") {
                PLUS | MINUS | TIMES | DIVISION | LT | EQ => {
                    //println!("parsing binop");
                    parse_binop(l)
                   // Ok(EBinop(Box::new(Binexp{op: })))
                },
                NEG => {
                    //println!("parsing unop");
                    parse_unop(l)
                },
                SEQ => {
                    parse_seq(l)
                },
                LET => {
                    parse_let(l)
                },
//                LET => parse_let(l)?,
                tok => parse_err!(l, format!("exp: unexpected token {:?}", tok))
            }
            // retrun statement here
        },
//      TT_VAL => { println!("tt_val so go to parse_val")},
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
        tok => parse_err!(l, format!("exp: unexpected token {:?}", tok))
    }
    //Ok(EI32(1))
}


fn parse_binop(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("binop: expected a token") {
        PLUS => {
            l.eat(PLUS);
            println!("parsing plus");
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            l.eat(RIGHT_PAREN);
            Ok(EBinop(Box::new(Binexp{op: BPlus, lhs: e2, rhs: e1})))
        },
        MINUS => {
            l.eat(MINUS);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            l.eat(RIGHT_PAREN);
            Ok(EBinop(Box::new(Binexp{op: BMinus, lhs: e2, rhs: e1})))
        },
        TIMES => {
            l.eat(TIMES);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            l.eat(RIGHT_PAREN);
            Ok(EBinop(Box::new(Binexp{op: BTimes, lhs: e2, rhs: e1})))
        },
        DIVISION => {
            l.eat(DIVISION);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            l.eat(RIGHT_PAREN);
            Ok(EBinop(Box::new(Binexp{op: BDivide, lhs: e2, rhs: e1})))
        },
        LT => {
            l.eat(LT);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            l.eat(RIGHT_PAREN);
            Ok(EBinop(Box::new(Binexp{op: BLt, lhs: e2, rhs: e1})))
        },
        EQ => {
            l.eat(EQ);
            let e1 = parse_exp(l)?;
            let e2 = parse_exp(l)?;
            l.eat(RIGHT_PAREN);
            Ok(EBinop(Box::new(Binexp{op: BEq, lhs: e2, rhs: e1})))
        },
        tok => parse_err!(l, format!("bionp: unexpected token {:?}", tok))
    }
}

fn parse_unop(l: &mut LexerState) -> Result<Exp, String> {
    match l.peek().expect("unop: expected a token") {
        NEG => {
            l.eat(NEG);
            let e1 = parse_exp(l)?;
            l.eat(RIGHT_PAREN);
            Ok(EUnop(Box::new(Unexp{op: UNeg, exp: e1})))
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
            l.eat(RIGHT_PAREN);
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
            l.eat(RIGHT_PAREN);
            Ok(ELet(Box::new(Letexp{var: v, exp1: e1, exp2: e2})))
        },
        tok => parse_err!(l, format!("let: unexpected token {:?}", tok))
    }
}

pub fn parse(s: &str) -> Result<Exp, String> {
    let mut l = LexerState::new(s);
    parse_prog(&mut l)
}
