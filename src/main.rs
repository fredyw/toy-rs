mod ast;
mod interpreter;
mod lexer;
mod parser;

use crate::interpreter::Environment;

fn main() {
    let code = "42";
    let lexer = lexer::Lexer::new(code);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse_expression(0);
    let mut env = Environment::new();
    let result = interpreter::eval_expression(ast, &mut env);
    println!("Runtime Result: {:?}", result);
}
