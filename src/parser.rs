use crate::ast::{BinaryOp, Expr, Literal, Stmt};
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

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while self.current_token != Token::Eof {
            match self.current_token {
                Token::Let | Token::Fn => {
                    statements.push(self.parse_statement());
                }
                // Expressions, e.g., "1 + 1"
                _ => {
                    let expr = self.parse_expression(0);
                    if self.current_token == Token::SemiColon {
                        self.advance(); // Eat `;`.
                        statements.push(Stmt::Expression(expr));
                    } else {
                        // If there is NO semicolon, it is only allowed if we are at EOF.
                        if self.current_token == Token::Eof {
                            statements.push(Stmt::ImplicitReturn(expr));
                        } else {
                            panic!("Expected ';' after expression");
                        }
                    }
                }
            }
        }

        statements
    }

    fn parse_unary(&mut self) -> Expr {
        if self.current_token == Token::Bang || self.current_token == Token::Minus {
            let op = match self.current_token {
                Token::Bang => crate::ast::UnaryOp::Not,
                Token::Minus => crate::ast::UnaryOp::Neg,
                _ => unreachable!(),
            };
            self.advance(); // Eat the `!` or `-`.
            let right = self.parse_unary();
            return Expr::Unary(op, Box::new(right));
        }
        self.parse_primary()
    }

    pub fn parse_expression(&mut self, min_precedence: u8) -> Expr {
        let mut lhs = self.parse_unary();
        while self.get_precedence() > min_precedence {
            let op_precedence = self.get_precedence();
            let op = self.get_binary_op().unwrap();
            self.advance(); // Eat the operator
            let rhs = self.parse_expression(op_precedence);
            lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
        }
        lhs
    }

    pub fn parse_statement(&mut self) -> Stmt {
        match self.current_token {
            // For example: let x = 123;
            Token::Let => self.parse_let_statement(),
            // For example: fn foo() {}
            Token::Fn => self.parse_function_statement(),
            // For example: a + 1;
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_block(&mut self) -> Expr {
        self.expect(Token::LBrace);
        let mut statements = Vec::new();
        let mut tail_expr = None;
        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            match self.current_token {
                Token::Let => {
                    statements.push(self.parse_let_statement());
                }
                Token::Fn => {
                    statements.push(self.parse_function_statement());
                }
                _ => {
                    let expr = self.parse_expression(0);
                    if self.current_token == Token::SemiColon {
                        // A statement. For example: "1 + 1;"
                        self.advance();
                        statements.push(Stmt::Expression(expr));
                    } else {
                        // An expression. For example: "1 + 1"
                        if self.current_token == Token::RBrace {
                            tail_expr = Some(Box::new(expr));
                        } else {
                            panic!("Expected ';' or '}}' after expression");
                        }
                    }
                }
            }
        }
        self.expect(Token::RBrace);
        Expr::Block(statements, tail_expr)
    }

    fn parse_primary(&mut self) -> Expr {
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
                self.advance(); // Eat the name
                if self.current_token == Token::LParen {
                    self.advance(); // Eat `(`.
                    let mut args = Vec::new();
                    if self.current_token != Token::RParen {
                        loop {
                            args.push(self.parse_expression(0));
                            if self.current_token == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen); // Eat `)`.
                    Expr::Call(name, args)
                } else {
                    Expr::Variable(name)
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression(0);
                self.expect(Token::RParen);
                expr
            }
            Token::LBrace => self.parse_block(),
            Token::If => self.parse_if_expression(),
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    fn get_precedence(&self) -> u8 {
        match self.current_token {
            Token::Star | Token::Slash => 20,         // * and / happen first
            Token::Plus | Token::Minus => 10,         // + and - happen after
            Token::EqEq | Token::Lt | Token::Gt => 5, // Comparisons happen last
            _ => 0,                                   // Not an operator
        }
    }

    fn get_binary_op(&self) -> Option<BinaryOp> {
        match self.current_token {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            Token::Star => Some(BinaryOp::Mul),
            Token::Slash => Some(BinaryOp::Div),
            Token::EqEq => Some(BinaryOp::Eq),
            Token::Lt => Some(BinaryOp::Lt),
            Token::Gt => Some(BinaryOp::Gt),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Stmt {
        self.advance(); // Eat the `let`.
        let name = match &self.current_token {
            Token::Identifier(n) => n.clone(),
            _ => panic!("Expected variable name after 'let'"),
        };
        self.advance(); // Eat the `name`.
        self.expect(Token::Eq);
        // Parse the value (RHS).
        let value = self.parse_expression(0);
        self.expect(Token::SemiColon);
        Stmt::Let(name, value)
    }

    fn parse_expression_statement(&mut self) -> Stmt {
        let expr = self.parse_expression(0);
        self.expect(Token::SemiColon);
        Stmt::Expression(expr)
    }

    fn parse_function_statement(&mut self) -> Stmt {
        self.advance(); // Eat `fn`.
        let name = match &self.current_token {
            Token::Identifier(n) => n.clone(),
            _ => panic!("Expected function name"),
        };
        self.advance();
        // Parse parameters (param1, param2, ...).
        self.expect(Token::LParen);
        let mut params = Vec::new();
        if self.current_token != Token::RParen {
            loop {
                match &self.current_token {
                    Token::Identifier(param_name) => {
                        params.push(param_name.clone());
                        self.advance();
                    }
                    _ => panic!("Expected parameter name"),
                }
                if self.current_token == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RParen);
        // Parse function body.
        let body = self.parse_block();
        Stmt::Fn(name, params, body)
    }

    fn parse_if_expression(&mut self) -> Expr {
        self.advance(); // Eat `if`.
        let condition = self.parse_expression(0);
        let then_branch = self.parse_block();
        let else_branch = if self.current_token == Token::Else {
            self.advance(); // Eat `else`.
            if self.current_token == Token::If {
                Some(Box::new(self.parse_if_expression()))
            } else {
                Some(Box::new(self.parse_block()))
            }
        } else {
            None
        };
        Expr::If(Box::new(condition), Box::new(then_branch), else_branch)
    }
}
