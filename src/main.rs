mod ast;
mod interpreter;
mod lexer;
mod parser;

use interpreter::{Environment, Value, eval_statement};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: toy-rs <filename.toy>");
        return;
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Could not read file");
    let lexer = lexer::Lexer::new(&code);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program();
    let mut env = Environment::new();
    let mut last_value = Value::Unit;
    for stmt in program {
        last_value = eval_statement(stmt, &mut env);
    }
    if last_value != Value::Unit {
        println!("{}", last_value);
    }
}
