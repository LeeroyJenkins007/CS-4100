//Author: Greg Jenkins

use std::io;

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
        Err(_err) => {
            println!("-1");
            panic!();},
        _ => user_input = user_input,
    }

    let command_vector = tokenize(user_input);
    
    //output to standard out, the result of the calculation
    print!("{}", analyze(command_vector));
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
                    } else { //if the value is not a digit or a negative sign
                        println!("-1");
                        panic!();
                    }
                }},
        }
    }
    command
}

//Purpose: compute value from passed array,
//if errors arise to panic.
fn analyze(token_vec: Vec<Term>) -> u32 {
    let mut stack_vector = vec![];
    let mut max_stack_value: u32 = 0;
    let mut current_stack_value: u32 = 0;
    let mut aux_stack: Vec<f32> = Vec::new();
    let mut current_val: f32 = 0.0;

    for term in token_vec {
        match term {
            Term::Value(int) => {
                stack_vector.push(int as f32);
                current_stack_value += 1;
                if current_stack_value > max_stack_value { max_stack_value = current_stack_value}},
            Term::Plus => {current_val = stack_vector.pop().unwrap() as f32 + stack_vector.pop().unwrap() as f32;
                stack_vector.push(current_val);
                current_stack_value -= 1;},
            Term::Subt => {current_val = -1.0 * stack_vector.pop().unwrap() as f32 + stack_vector.pop().unwrap() as f32;
                stack_vector.push(current_val);
                current_stack_value -= 1;},
            Term::Mult => {current_val = stack_vector.pop().unwrap() as f32 * stack_vector.pop().unwrap() as f32;
                stack_vector.push(current_val);
                current_stack_value -= 1;},
            Term::Div => {
                let zero_check = stack_vector.pop().unwrap() as f32;
                if zero_check == 0.0{
                    println!("-1");
                    panic!();
                } else {
                    current_val = stack_vector.pop().unwrap() as f32 / zero_check;
                    stack_vector.push(current_val);
                    current_stack_value -= 1;
                }},
            Term::Save => {
                if stack_vector.len() <= 0{
                    println!("-1");
                    panic!();
                }else {
                    aux_stack.push(current_val);
                    current_stack_value += 1;
                    if current_stack_value > max_stack_value { max_stack_value = current_stack_value}
                }},
            Term::Restore => {
                stack_vector.push(aux_stack.pop().unwrap())},
            Term::Done => {
                if stack_vector.len() <= 0{
                    println!("-1");
                    panic!();
                }else{
                    return max_stack_value
                }},
        }
    }
    println!("-1");
    panic!();
    //max_stack_value
}

