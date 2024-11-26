use std::collections::HashMap;
use std::fs;
use std::env;
use crate::interpreter::interpret;
pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: nim <file.ns>");
        return;
    }

    let filename = &args[1];

    match fs::read_to_string(filename) {
        Ok(contents) => {
            let mut variables = HashMap::new();

            for statement in contents.split_inclusive(';') {
                let trimmed = statement.trim();
                if !trimmed.is_empty() {
                    interpret(trimmed, &mut variables);
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading file {}: {}", filename, err);
        }
    }
}