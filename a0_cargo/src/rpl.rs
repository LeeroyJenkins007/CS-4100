//Author: Greg Jenkins
//Binary Operators
//b ::= + | - | * | /
 
//Terms
//t ::= n       //32-bit signed integers, e.g., -1, 2, 256, 0, ...
//    | b       //Binary operator
//    | save    //Pop from the main stack, pushing the value onto an auxiliary stack 
//    | restore //Pop from the auxiliary stack, pushing the value onto the main stack

//RPN Programs
//p ::= t_0 t_1 ... t_n done

use std::io;

#[derive(Debug)]
enum Term {
    Value (i32),
    Plus,
    Subt,
    Div,
    Mult,
    Save,
    Restore,
    Done,
}


fn main() {
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Err(_err) => panic!(),
        _ => user_input = user_input,
    }

    let command_vector = tokenize(user_input);
    
    //output to standard out, the result of the calculation
    print!("{}", calculate(command_vector));
}

fn tokenize(user_input: String) -> Vec<Term> {
    user_input.trim();

    let mut command = vec![];
    let mut negative = false;

    for iter in user_input.split_whitespace() {
        match iter.as_ref(){
            "+" => command.push(Term::Plus),
            "-" => command.push(Term::Subt),
            "/" => command.push(Term::Div),
            "*" => command.push(Term::Mult),
            "save" => command.push(Term::Save),
            "restore" => command.push(Term::Restore),
            "done" => {
                command.push(Term::Done);
                break;
            }
            _ => { 
                //Horrible code, need improvement
                let mut temp = iter.clone();
                let mut temp2 = iter.clone();
                
                let my_int = temp2.parse::<i32>().unwrap();
                
                for c in temp.chars(){
                    if c.is_digit(10) || c == '-'{
                        if c == '-'{
                            negative = true;
                        }else if negative{
                            command.push(Term::Value(my_int));
                        }else {
                            command.push(Term::Value(my_int));
                        }
                    } else {
                        panic!();
                    }
                }},
        }
    }
    command
}

//Purpose: compute value from passed array,
//if errors arise to panic.
fn calculate(token_vec: Vec<Term>) -> f32 {
    let mut stack_vector = vec![];
    let mut aux_stack: Vec<f32> = Vec::new();
    let mut current_val: f32 = 0.0;

    for term in token_vec {
        match term {
            Term::Value(int) => stack_vector.push(int as f32),
            Term::Plus => {current_val = stack_vector.pop().unwrap() as f32 + stack_vector.pop().unwrap() as f32;
                stack_vector.push(current_val);},
            Term::Subt => {current_val = -1.0 * stack_vector.pop().unwrap() as f32 + stack_vector.pop().unwrap() as f32;
                stack_vector.push(current_val);},
            Term::Mult => {current_val = stack_vector.pop().unwrap() as f32 * stack_vector.pop().unwrap() as f32;
                stack_vector.push(current_val);},
            Term::Div => {
                let zero_check = stack_vector.pop().unwrap() as f32;
                if zero_check == 0.0{
                    panic!();
                } else {
                    current_val = stack_vector.pop().unwrap() as f32 / zero_check;
                    stack_vector.push(current_val);
                }},
            Term::Save => {
                if stack_vector.len() <= 0{
                    panic!();
                }else {
                    aux_stack.push(current_val)
                }},
            Term::Restore => {stack_vector.push(aux_stack.pop().unwrap())},
            Term::Done => {
                if stack_vector.len() <= 0{
                    panic!();
                }},
        }
    }
    current_val
}

