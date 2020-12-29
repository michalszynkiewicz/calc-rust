use std::collections::{HashMap, LinkedList};
use std::env;
use std::num::ParseFloatError;
use std::str::Chars;
use std::borrow::BorrowMut;
use lazy_static::lazy_static;

struct Operation {
    value: Option<f64>,
    operation: i16,
}

const ADD: i16 = 0;
const SUBTRACT: i16 = 1;
const MULTIPLY: i16 = 2;
const DIVIDE: i16 = 3;
const NEGATE: i16 = 4;

lazy_static! {
    static ref PRIORITY: HashMap<i16, u8> = [(ADD, 1), (SUBTRACT, 1), (MULTIPLY, 2), (DIVIDE, 2), (NEGATE, 5)].iter().cloned().collect();
}

fn add(value_stack: &mut LinkedList<f64>) -> f64 {
    let one = value_stack.pop_back();
    let two = value_stack.pop_back();
    return one.unwrap() + two.unwrap();
}

fn subtract(value_stack: &mut LinkedList<f64>) -> f64 {
    let one = value_stack.pop_back();
    let two = value_stack.pop_back();
    return two.unwrap() - one.unwrap();
}

fn multiply(value_stack: &mut LinkedList<f64>) -> f64 {
    let one = value_stack.pop_back();
    let two = value_stack.pop_back();
    return one.unwrap() * two.unwrap();
}

fn divide(value_stack: &mut LinkedList<f64>) -> f64 {
    let one = value_stack.pop_back();
    let two = value_stack.pop_back();
    return two.unwrap() / one.unwrap();
}

fn negate(value_stack: &mut LinkedList<f64>) -> f64 {
    let one = value_stack.pop_back();
    return 0. - one.unwrap();
}

fn parse_expression(task: &mut Chars, stack: &mut LinkedList<Operation>) {
    let mut current: String = String::new();
    let mut local_operation_stack: LinkedList<i16> = LinkedList::new();

    let mut negation = true; // if true, `-` in the current context means negation

    loop {
        let maybe_next = task.next();
        if maybe_next.is_some() {
            let x = maybe_next.unwrap();
            if x == ')' {
                break;
            } else if x == '(' {
                parse_expression(task, stack)
            } else if x >= '0' && x <= '9' || x == '.' || x == ',' {
                current.push(x);
                negation = false;
            } else {
                let val = parse_value(&current);
                if val.is_ok() {
                    let value = val.unwrap();
                    current = String::new();
                    stack.push_back(Operation { value: Some(value), operation: -1 });
                }
                if x == '*' {
                    negation = true;
                    add_operation(stack, &mut local_operation_stack, MULTIPLY);
                } else if x == '/' {
                    negation = true;
                    add_operation(stack, &mut local_operation_stack, DIVIDE);
                } else if x == '+' {
                    negation = true;
                    add_operation(stack, &mut local_operation_stack, ADD);
                } else if x == '-' {
                    if negation {
                        add_operation(stack, &mut local_operation_stack, NEGATE);
                    } else {
                        negation = true;
                        add_operation(stack, &mut local_operation_stack, SUBTRACT);
                    }
                }
            }
        } else {
            break;
        }
    }

    if !current.is_empty() {
        let val = parse_value(&current);
        if val.is_ok() {
            let value = val.unwrap();
            stack.push_back(Operation { value: Some(value), operation: -1 });
        } else {
            println!("failed to parse value {}", current);
            panic!("failed to parse value");
        }
    }

    loop {
        let last_operation = local_operation_stack.pop_back();
        if last_operation.is_none() {
            break;
        }
        stack.push_back(Operation { operation: last_operation.unwrap(), value: None });
    }
}

fn parse_value(p0: &String) -> Result<f64, ParseFloatError> {
    return p0.replace(",", ".").parse::<f64>();
}

fn add_operation(stack: &mut LinkedList<Operation>, local_operation_stack: &mut LinkedList<i16>, operation: i16) {
    let back = local_operation_stack.pop_back();
    if back.is_some() {
        if PRIORITY.get(&back.unwrap()).unwrap() >= PRIORITY.get(&operation).unwrap() {
            let old_operation = back.unwrap();
            stack.push_back(Operation { operation: old_operation.clone(), value: None })
        } else {
            local_operation_stack.push_back(back.unwrap());
        }
    }
    local_operation_stack.push_back(operation);
}

fn remove_whitespace(s: &str) -> String {
    // taken from https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-string
    s.split_whitespace().collect()
}


fn main() {
    let merged_args = env::args().into_iter().skip(1).fold(String::new(), |a, b| a + " " + b.as_str());
    let expression = remove_whitespace(merged_args.as_str());
    let mut stack: LinkedList<Operation> = LinkedList::new();
    parse_expression(expression.chars().borrow_mut(), &mut stack);
    /*print!("parsing results: ");
    stack.iter().for_each(
        |oper| {
            if oper.value.is_some() {
                print!(" {} ", oper.value.unwrap())
            } else {
                match oper.operation {
                    ADD => print!("+"),
                    SUB => print!("-"),
                    MUL => print!("*"),
                    DIV => print!("/"),
                    _ => println!("unknown")
                }
            }
        }
    );
    println!();*/

    let mut value_stack: LinkedList<f64> = LinkedList::new();
    loop {
        let option = stack.pop_front();
        if option.is_none() {
            break;
        } else {
            let operation = option.unwrap();
            if operation.value.is_some() {
                value_stack.push_back(operation.value.unwrap())
            } else {
                let result: f64 = match operation.operation {
                    NEGATE => negate(&mut value_stack),
                    ADD => add(&mut value_stack),
                    SUBTRACT => subtract(&mut value_stack),
                    MULTIPLY => multiply(&mut value_stack),
                    DIVIDE => divide(&mut value_stack),
                    _ => panic!("unsupported operation")
                };
                value_stack.push_back(result)
            }
        }
    }

    println!("{}", value_stack.pop_front().unwrap());
}

