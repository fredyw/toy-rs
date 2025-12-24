use crate::ast::{BinaryOp, Expr, Literal, Stmt, UnaryOp};
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let first_token = lexer.next_token();
        Parser {
            lexer,
            current_token: first_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) {
        if self.current_token == expected {
            self.advance();
        } else {
            panic!("Expected {:?}, but got {:?}", expected, self.current_token);
        }
    }

    pub fn parse_primary(&mut self) -> Expr {
        let token = self.current_token.clone();
        match token {
            Token::Int(val) => {
                self.advance();
                Expr::Literal(Literal::Int(val))
            }
            Token::Float(val) => {
                self.advance();
                Expr::Literal(Literal::Float(val))
            }
            Token::Str(val) => {
                self.advance();
                Expr::Literal(Literal::Str(val))
            }
            Token::True => {
                self.advance();
                Expr::Literal(Literal::Bool(true))
            }
            Token::False => {
                self.advance();
                Expr::Literal(Literal::Bool(false))
            }
            Token::Identifier(name) => {
                self.advance();
                Expr::Variable(name)
            }
            _ => panic!("Unexpected token in expression: {:?}", token),
        }
    }
}
