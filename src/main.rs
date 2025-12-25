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
        add(x, 20)
    ";
    let wrapped_code = format!("{{ {} }}", code);
    let lexer = lexer::Lexer::new(&wrapped_code);
    let mut parser = parser::Parser::new(lexer);
    let ast = parser.parse_expression(0);
    let mut env = interpreter::Environment::new();
    let result = interpreter::eval_expression(ast, &mut env);
    println!("Final Result: {:?}", result);
}
