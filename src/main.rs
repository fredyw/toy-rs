use crate::interpreter::Environment;

mod ast;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let code = "
        fn add(a, b) {
            a + b
        }

        let x = 10;
        let result = add(x, 20);
        result
    ";
    let lexer = lexer::Lexer::new(code);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program();
    let mut env = Environment::new();
    let mut last_value = interpreter::Value::Unit;
    for stmt in program {
        last_value = interpreter::eval_statement(stmt, &mut env);
    }
    println!("Program result: {:?}", last_value);
}
