mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = "let result = if x > 5 { 100 } else { 0 };";
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_statement();
    println!("AST: {:#?}", ast);
}
