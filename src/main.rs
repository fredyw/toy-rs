mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = "let x = 123 + 456;";
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_statement();
    println!("AST: {:#?}", ast);
}
