use crate::ast::{BinaryOp, Expr, Literal, Stmt};
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let mut first_token = lexer.next_token();
        while let Token::Comment = first_token {
            first_token = lexer.next_token();
        }
        Parser {
            lexer,
            current_token: first_token,
        }
    }

    fn advance(&mut self) {
        loop {
            self.current_token = self.lexer.next_token();
            if !matches!(self.current_token, Token::Comment) {
                break;
            }
        }
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
                Token::While => {
                    statements.push(self.parse_while_statement());
                }
                // Expressions (e.g., "1 + 1") or Assignments (e.g. "x += 1")
                _ => {
                    statements.push(self.parse_expression_statement());
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
            // For example: while cond {}
            Token::While => self.parse_while_statement(),
            // For example: a + 1;
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_while_statement(&mut self) -> Stmt {
        self.advance(); // Eat `while`.
        let condition = self.parse_expression(0);
        let body = self.parse_block();
        Stmt::While(condition, body)
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
                Token::While => {
                    statements.push(self.parse_while_statement());
                }
                _ => {
                    let expr = self.parse_expression(0);

                    if matches!(
                        self.current_token,
                        Token::PlusEq | Token::MinusEq | Token::StarEq | Token::SlashEq | Token::Eq
                    ) {
                        let name = match expr {
                            Expr::Variable(n) => n,
                            _ => panic!(
                                "Invalid assignment target. Only variables can be assigned to."
                            ),
                        };
                        let op = match self.current_token {
                            Token::PlusEq => BinaryOp::Add,
                            Token::MinusEq => BinaryOp::Sub,
                            Token::StarEq => BinaryOp::Mul,
                            Token::SlashEq => BinaryOp::Div,
                            Token::Eq => {
                                self.advance();
                                let right = self.parse_expression(0);
                                self.expect(Token::SemiColon);
                                statements.push(Stmt::Assign(name, right));
                                continue;
                            }
                            _ => unreachable!(),
                        };
                        self.advance(); // Eat the operator (+=, etc).
                        let right = self.parse_expression(0);
                        self.expect(Token::SemiColon);
                        let new_value_expr = Expr::Binary(
                            Box::new(Expr::Variable(name.clone())),
                            op,
                            Box::new(right),
                        );
                        statements.push(Stmt::Assign(name, new_value_expr));
                        continue;
                    }

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
            Token::And => 3,
            Token::Or => 1,
            _ => 0, // Not an operator
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
            Token::And => Some(BinaryOp::And),
            Token::Or => Some(BinaryOp::Or),
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
        if matches!(
            self.current_token,
            Token::PlusEq | Token::MinusEq | Token::StarEq | Token::SlashEq | Token::Eq
        ) {
            let name = match expr {
                Expr::Variable(n) => n,
                _ => panic!("Invalid assignment target. Only variables can be assigned to."),
            };
            if self.current_token == Token::Eq {
                self.advance();
                let right = self.parse_expression(0);
                self.expect(Token::SemiColon);
                return Stmt::Assign(name, right);
            }
            let op = match self.current_token {
                Token::PlusEq => BinaryOp::Add,
                Token::MinusEq => BinaryOp::Sub,
                Token::StarEq => BinaryOp::Mul,
                Token::SlashEq => BinaryOp::Div,
                _ => unreachable!(),
            };
            self.advance(); // Eat the operator (+=, etc).
            let right = self.parse_expression(0);
            self.expect(Token::SemiColon);
            let new_value_expr =
                Expr::Binary(Box::new(Expr::Variable(name.clone())), op, Box::new(right));
            return Stmt::Assign(name, new_value_expr);
        }

        // Allow omitting semicolon for block-like expressions (If, Block)
        let is_block_like = matches!(expr, Expr::If(..) | Expr::Block(..));

        if self.current_token == Token::SemiColon {
            self.advance();
            Stmt::Expression(expr)
        } else if is_block_like {
            Stmt::Expression(expr)
        } else if self.current_token == Token::Eof {
            Stmt::ImplicitReturn(expr)
        } else {
            panic!("Expected ';' after expression");
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOp, Expr, Literal, Stmt, UnaryOp};
    use crate::lexer::Lexer;

    fn parse_helper(input: &str) -> Vec<Stmt> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program()
    }

    #[test]
    fn test_let_statement() {
        let input = "let x = 5;";
        let statements = parse_helper(input);
        assert_eq!(statements.len(), 1);
        match &statements[0] {
            Stmt::Let(name, expr) => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Literal(Literal::Int(val)) => assert_eq!(*val, 5),
                    _ => panic!("Expected integer literal"),
                }
            }
            _ => panic!("Expected Let statement"),
        }
    }

    #[test]
    fn test_expression_precedence() {
        let input = "1 + 2 * 3;";
        let statements = parse_helper(input);
        assert_eq!(statements.len(), 1);
        match &statements[0] {
            Stmt::Expression(expr) => {
                // Should be (1 + (2 * 3))
                match expr {
                    Expr::Binary(lhs, op, rhs) => {
                        assert_eq!(*op, BinaryOp::Add);
                        match &**lhs {
                            Expr::Literal(Literal::Int(v)) => assert_eq!(*v, 1),
                            _ => panic!("Left side should be 1"),
                        }
                        match &**rhs {
                            Expr::Binary(r_lhs, r_op, r_rhs) => {
                                assert_eq!(*r_op, BinaryOp::Mul);
                                match &**r_lhs {
                                    Expr::Literal(Literal::Int(v)) => assert_eq!(*v, 2),
                                    _ => panic!("Inner left should be 2"),
                                }
                                match &**r_rhs {
                                    Expr::Literal(Literal::Int(v)) => assert_eq!(*v, 3),
                                    _ => panic!("Inner right should be 3"),
                                }
                            }
                            _ => panic!("Right side should be multiplication"),
                        }
                    }
                    _ => panic!("Expected Binary expression"),
                }
            }
            _ => panic!("Expected Expression statement"),
        }
    }

    #[test]
    fn test_implicit_return() {
        let input = "let x = 10; x + 5";
        let statements = parse_helper(input);
        assert_eq!(statements.len(), 2);
        match &statements[1] {
            Stmt::ImplicitReturn(expr) => match expr {
                Expr::Binary(lhs, op, rhs) => {
                    assert_eq!(*op, BinaryOp::Add);
                    match &**lhs {
                        Expr::Variable(name) => assert_eq!(name, "x"),
                        _ => panic!("Expected variable"),
                    }
                    match &**rhs {
                        Expr::Literal(Literal::Int(v)) => assert_eq!(*v, 5),
                        _ => panic!("Expected 5"),
                    }
                }
                _ => panic!("Expected Binary expression"),
            },
            _ => panic!("Expected ImplicitReturn statement"),
        }
    }

    #[test]
    fn test_unary_expression() {
        let input = "-5;";
        let statements = parse_helper(input);
        match &statements[0] {
            Stmt::Expression(Expr::Unary(op, expr)) => {
                assert_eq!(*op, UnaryOp::Neg);
                match &**expr {
                    Expr::Literal(Literal::Int(v)) => assert_eq!(*v, 5),
                    _ => panic!("Expected 5"),
                }
            }
            _ => panic!("Expected Unary expression"),
        }
    }

    #[test]
    fn test_only_comments() {
        let input = "// first comment\n// second comment";
        let statements = parse_helper(input);
        assert_eq!(statements.len(), 0);
    }

    #[test]
    fn test_while_statement() {
        let input = "while true { 1 }";
        let statements = parse_helper(input);
        assert_eq!(statements.len(), 1);
        match &statements[0] {
            Stmt::While(cond, body) => {
                match cond {
                    Expr::Literal(Literal::Bool(b)) => assert_eq!(*b, true),
                    _ => panic!("Expected boolean literal"),
                }
                match body {
                    Expr::Block(stmts, tail) => {
                        assert_eq!(stmts.len(), 0);
                        match tail {
                            Some(expr) => match &**expr {
                                Expr::Literal(Literal::Int(v)) => assert_eq!(*v, 1),
                                _ => panic!("Expected 1"),
                            },
                            None => panic!("Expected tail expression"),
                        }
                    }
                    _ => panic!("Expected block"),
                }
            }
            _ => panic!("Expected While statement"),
        }
    }

    #[test]
    fn test_assignment() {
        // Simple assignment
        let input = "x = 5;";
        let statements = parse_helper(input);
        match &statements[0] {
            Stmt::Assign(name, expr) => {
                assert_eq!(name, "x");
                match expr {
                    Expr::Literal(Literal::Int(v)) => assert_eq!(*v, 5),
                    _ => panic!("Expected 5"),
                }
            }
            _ => panic!("Expected Assign statement"),
        }

        // Compound assignment
        let input = "x += 1;";
        let statements = parse_helper(input);
        match &statements[0] {
            Stmt::Assign(name, expr) => {
                assert_eq!(name, "x");
                // x += 1 parses to x = x + 1
                match expr {
                    Expr::Binary(lhs, op, rhs) => {
                        assert_eq!(*op, BinaryOp::Add);
                        match &**lhs {
                            Expr::Variable(n) => assert_eq!(n, "x"),
                            _ => panic!("Expected variable x"),
                        }
                        match &**rhs {
                            Expr::Literal(Literal::Int(v)) => assert_eq!(*v, 1),
                            _ => panic!("Expected 1"),
                        }
                    }
                    _ => panic!("Expected binary expression"),
                }
            }
            _ => panic!("Expected Assign statement"),
        }
    }

    #[test]
    fn test_logical_precedence() {
        let input = "true || false && false;";
        let statements = parse_helper(input);
        match &statements[0] {
            Stmt::Expression(expr) => {
                // Expected: true || (false && false)
                match expr {
                    Expr::Binary(lhs, op, rhs) => {
                        assert_eq!(*op, BinaryOp::Or);
                        match &**lhs {
                            Expr::Literal(Literal::Bool(b)) => assert!(b),
                            _ => panic!("Expected true"),
                        }
                        match &**rhs {
                            Expr::Binary(r_lhs, r_op, r_rhs) => {
                                assert_eq!(*r_op, BinaryOp::And);
                                match &**r_lhs {
                                    Expr::Literal(Literal::Bool(b)) => assert!(!b),
                                    _ => panic!("Expected false"),
                                }
                                match &**r_rhs {
                                    Expr::Literal(Literal::Bool(b)) => assert!(!b),
                                    _ => panic!("Expected false"),
                                }
                            }
                            _ => panic!("Expected AND expression on RHS"),
                        }
                    }
                    _ => panic!("Expected OR expression"),
                }
            }
            _ => panic!("Expected Expression statement"),
        }
    }
}
