mod ast;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let code = "5 * 2 + 10";
    let lexer = lexer::Lexer::new(code);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse_expression(0);
    let mut env = interpreter::Environment::new();
    let result = interpreter::eval_expression(ast, &mut env);
    println!("Runtime Result: {:?}", result);
}
