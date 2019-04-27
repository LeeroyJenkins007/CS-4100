use regex::Regex;


#[derive(Debug,PartialEq)]
pub enum Token {
    I32_VAL(i32),
    TRUE_VAL,
    FALSE_VAL,
    TT_VAL,
    ID(String),
    RETARROW,
    LET,
    EXPSTART,
    SEQ,
    ALLOC,
    SET,
    GET,
    COND,
    FUNDECL,
    FUNPTR,
    CALL,
    LEFT_PAREN,
    RIGHT_PAREN,
    PRINT,
    SPAWN,
    NEG,
    PLUS,
    TIMES,
    MINUS,
    DIVISION,
    LT,
    EQ,
    ARRAY_TYPE,
    UNIT_TYPE,
    BOOL_TYPE,
    INT_TYPE,
}

#[derive(Debug,Clone)]
pub struct LineInfo {
    pub line_no: u64,
    pub col_no: u64,
}

impl LineInfo {
    fn incr_line(&mut self, n: u64) {
        self.col_no = 0;
        self.line_no = self.line_no + n
    }

    fn incr_col(&mut self, n: u64) {
        self.col_no = self.col_no + n
    }
}


macro_rules! lex_upd {
    ( $l:expr, $no_chars: expr, $tok: expr) => {{
        $l.info.incr_col($no_chars);
        $l.rest = $l.rest.split_at($no_chars).1;
        if $l.comment_depth > 0 { lex($l) }
        else { Ok($tok) }
    }}
}


fn lex<'a>(l: &mut LexerState<'a>) -> Result<Token, String> {
    let s = l.rest;
    let int_regex = Regex::new(r"^\A[[:digit:]]+").unwrap();
    let var_regex = Regex::new(r"^\A[[:alpha:]]+").unwrap();

    //Comments
    if s.starts_with("/*") {
        l.comment_depth = l.comment_depth + 1;
        l.rest = s.split_at(2).1;
        lex(l)
    }
    else if s.starts_with("*/") {
        l.comment_depth = l.comment_depth - 1;
        l.rest = s.split_at(2).1;
        lex(l)
    }

    
   //Whitespace
   else if s.starts_with(" ") {
       l.info.incr_col(1);
       l.rest = s.split_at(1).1;
       lex(l)
   }
   else if s.starts_with("\t") {
       l.info.incr_col(1);
       l.rest = s.split_at(1).1;
       lex(l)
   }

   //Newline sharacters
   else if s.starts_with("\r\n") {
       l.info.incr_line(1);
       l.rest = s.split_at(2).1;
       lex(l)
   }
   else if s.starts_with("\r") {
       l.info.incr_line(1);
       l.rest = s.split_at(1).1;
       lex(l)
   }
   else if s.starts_with("\n") {
       l.info.incr_line(1);
       l.rest = s.split_at(1).1;
       lex(l)
   }

   //the rest
   else if s.starts_with("(") { lex_upd!(l, 1, Token::LEFT_PAREN)}
   else if s.starts_with(")") { lex_upd!(l, 1, Token::RIGHT_PAREN)}
   else if s.starts_with("+") { lex_upd!(l, 1, Token::PLUS)}
   else if s.starts_with("*") { lex_upd!(l, 1, Token::TIMES)}
   else if s.starts_with("->") { lex_upd!(l, 2, Token::RETARROW)}
   else if s.starts_with("-") { lex_upd!(l, 1, Token::MINUS)}
   else if s.starts_with("/") { lex_upd!(l, 1, Token::DIVISION)}
   else if s.starts_with("<") { lex_upd!(l, 1, Token::LT)}
   else if s.starts_with("==") { lex_upd!(l, 2, Token::EQ)}
   else if s.starts_with("%") { lex_upd!(l, 1, Token::EXPSTART)}
   else if s.starts_with("neg") { lex_upd!(l, 3, Token::NEG)}
   else if s.starts_with("let") { lex_upd!(l, 3, Token::LET)}
   else if s.starts_with("seq") { lex_upd!(l, 3, Token::SEQ)}
   else if s.starts_with("alloc") { lex_upd!(l, 5, Token::ALLOC)}
   else if s.starts_with("set") { lex_upd!(l, 3, Token::SET)}
   else if s.starts_with("get") { lex_upd!(l, 3, Token::GET)}
   else if s.starts_with("cond") { lex_upd!(l, 4, Token::COND)}
   else if s.starts_with("funptr") { lex_upd!(l, 6, Token::FUNPTR)}
   else if s.starts_with("fun") { lex_upd!(l, 3, Token::FUNDECL)}
   else if s.starts_with("call") { lex_upd!(l, 4, Token::CALL)}
   else if s.starts_with("print") { lex_upd!(l, 5, Token::PRINT)}
   else if s.starts_with("spawn") { lex_upd!(l, 5, Token::SPAWN)}
   else if s.starts_with("i32") { lex_upd!(l, 3, Token::INT_TYPE)}
   else if s.starts_with("bool") { lex_upd!(l, 4, Token::BOOL_TYPE)}
   else if s.starts_with("unit") { lex_upd!(l, 4, Token::UNIT_TYPE)}
   else if s.starts_with("array") { lex_upd!(l, 5, Token::ARRAY_TYPE)}
   else if s.starts_with("true") { lex_upd!(l, 4, Token::TRUE_VAL)}
   else if s.starts_with("false") { lex_upd!(l, 5, Token::FALSE_VAL)}
   else if s.starts_with("tt") { lex_upd!(l, 2, Token::TT_VAL)}
    
   //If int, variable, fun name
   else if int_regex.is_match(s) {
       match int_regex.find(s){
           Some(mat) => {
               assert_eq!(mat.start(), 0);
               let (n, rest) = s.split_at(mat.end());
               l.info.incr_col(mat.end() as u64);
               l.rest = rest;
               if l.comment_depth > 0 { lex(l) }
               else { Ok(Token::I32_VAL(n.parse::<i32>().unwrap()))}
           },
           None => {
               if s.len() > 0 {
                   if l.comment_depth > 0 {
                       l.info.incr_col(1);
                       l.rest = l.rest.split_at(1).1;
                       lex(l)
                   } else {
                       Err(format!("unexpected token '{}'", s.split_at(1).0))
                   }
               } else {
                   Err(format!("unexpected end of program"))
               }
           },
       }
   }
   else {
       match var_regex.find(s) {
           Some(mat) => {
               assert_eq!(mat.start(), 0);
               let (n, rest) = s.split_at(mat.end());
               //let pr = n.clone();
               //println!("VAR: {:?}", pr);
               l.info.incr_col(mat.end() as u64);
               l.rest = rest;
               if l.comment_depth > 0 { lex(l) }
               else { Ok(Token::ID(n.to_owned()))}
           },
           None => {
            if s.len() > 0 {
                if l.comment_depth > 0 {
                    l.info.incr_col(1);
                    l.rest = l.rest.split_at(1).1;
                    lex(l)
                } else {
                    Err(format!(r"unexpected token '{}'", s.split_at(1).0))
                }
            } else {
                Err(format!("unexpected end of program"))
            }
           },
       }
   }




}

#[derive(Clone)]
pub struct LexerState<'a> {
    comment_depth: u64,
    pub rest: &'a str,
    pub info: LineInfo,
}

impl<'a> LexerState<'a> {
    pub fn new(s: &'a str) -> Self {
        LexerState{
            comment_depth: 0,
            rest: s.trim_end(),
            info: LineInfo{line_no: 1, col_no: 0},
        }
    }

    pub fn peek(self: &mut LexerState<'a>) -> Option<Token> {
        let revert = self.clone();
        match lex(self) {
            Ok(tok) => {
                *self = revert;
                Some(tok)
            },
            Err(err) => {
                eprintln!("lexer error: {} at {}:{}",
                          err, self.info.line_no, self.info.col_no);
                None
            }
        }
    }

    pub fn next(self: &mut LexerState<'a>) -> Option<Token> {
        match lex(self) {
            Ok(tok) => Some(tok),
            Err(err) => {
                eprintln!(r"lexer error: {} at {}:{}",
                          err, self.info.line_no, self.info.col_no);
                None
            }
        }
    }

    pub fn eat(self: &mut LexerState<'a>, expected: Token) -> Option<()> {
        if let Some(t) = self.next() {
            if t == expected { Some(()) }
            else { None }}
        else { None }
    }
}
