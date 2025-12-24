mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = "3.14"; // A very simple program
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_primary();
    println!("AST: {:?}", ast);
}
