mod ast;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let code = "
        let x = 10;
        let y = {
            let x = 5;
            x + 2
        };
        x + y
    ";
    let lexer = lexer::Lexer::new(code);
    let mut parser = parser::Parser::new(lexer);
    let wrapped_code = format!("{{ {} }}", code);
    let lexer = lexer::Lexer::new(&wrapped_code);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse_expression(0); // Parse the big block
    let mut env = interpreter::Environment::new();
    let result = interpreter::eval_expression(ast, &mut env);
    println!("Runtime Result: {:?}", result);
}
