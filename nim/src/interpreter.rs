use std::collections::HashMap;
use crate::lexer::{tokenize, Token};
use crate::parser::{parse, Expr};
use std::f64;
use std::io;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Array(Vec<Value>),
}

pub fn evaluate(expr: &Expr, variables: &mut HashMap<String, Value>) -> Value {
    match expr {
        Expr::Number(value) => Value::Number(*value),
        Expr::String(value) => Value::String(value.clone()),
        Expr::Variable(name) => {
            if let Some(value) = variables.get(name) {
                value.clone()
            } else {
                panic!("Undefined variable: {}", name);
            }
        }
        Expr::Array(elements) => {
            let mut arr = Vec::new();
            for element in elements {
                arr.push(evaluate(element, variables));
            }
            Value::Array(arr)
        }
        Expr::ArrayAccess { array, index } => {
            let arr = evaluate(array, variables);
            if let Value::Array(arr) = arr {
                let idx = match evaluate(index, variables) {
                    Value::Number(n) => n as usize,
                    _ => panic!("Index should be a number"),
                };
                arr.get(idx).cloned().unwrap_or(Value::Number(f64::NAN))
            } else {
                panic!("Expected an array for array access");
            }
        }
        Expr::Assignment { name, value } => {
            let e_value = evaluate(value, variables);
            variables.insert(name.clone(), e_value.clone());
            e_value
        }
        Expr::BinaryOp { left, op, right } => {
            let left_val = evaluate(left, variables);
            let right_val = evaluate(right, variables);

            match (left_val, right_val) {
                (Value::Number(left), Value::Number(right)) => match op {
                    '+' => Value::Number(left + right),
                    '-' => Value::Number(left - right),
                    '*' => Value::Number(left * right),
                    '/' => Value::Number(left / right),
                    '^' => Value::Number(left.powf(right)),
                    _ => panic!("Unsupported operator: {}", op),
                },
                _ => panic!("Cannot operate on non-numeric values"),
            }
        }
        Expr::FunctionCall { name, args } => {
            if name == "print" {
                for arg in args {
                    match evaluate(arg, variables) {
                        Value::Number(n) => print!("{} ", n),
                        Value::String(s) => print!("{} ", s),
                        _ => panic!("Unsupported print argument type"),
                    }
                }
                println!();
                Value::Number(0.0)
            } else if name == "write" {
                if !args.is_empty() {
                    panic!("write function does not take any arguments.");
                }
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed = input.trim();
                if let Ok(num) = trimmed.parse::<f64>() {
                    Value::Number(num)
                } else {
                    Value::String(trimmed.to_string())
                }
            } else {
                panic!("Unknown function: {}", name);
            }
        }
    }
}

pub fn interpret(input: &str, variables: &mut HashMap<String, Value>) {
    let tokens = tokenize(input);
    let has_semicolon = tokens.iter().any(|t| matches!(t, Token::Semicolon));
    let expr = parse(&tokens);

    let result = evaluate(&expr, variables);

    if has_semicolon {
        match result {
            Value::Number(n) => {
                variables.insert("_tnum_".to_string(), Value::Number(n));
            }
            Value::String(s) => {
                variables.insert("_tstr_".to_string(), Value::String(s.clone()));
            }
            Value::Array(arr) => {
                variables.insert("_tarr_".to_string(), Value::Array(arr.clone()));
            }
        }
    } else if !has_semicolon {
        match result {
            Value::Number(n) => println!("{}", n),
            Value::String(s) => println!("{}", s),
            Value::Array(arr) => println!("{:?}", arr)
        }
    }
}