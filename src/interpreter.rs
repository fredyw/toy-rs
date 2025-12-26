use crate::ast;
use crate::ast::BinaryOp;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Unit,
    Function(Vec<String>, ast::Expr),
    NativeFunc(fn(Vec<Value>) -> Value),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(l), Value::Int(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Str(l), Value::Str(r)) => l == r,
            (Value::Unit, Value::Unit) => true,
            (Value::Function(lp, lb), Value::Function(rp, rb)) => lp == rp && lb == rb,
            (Value::NativeFunc(l), Value::NativeFunc(r)) => *l as usize == *r as usize,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Unit => write!(f, "()"),
            Value::Function(params, _) => {
                write!(f, "<fn ({})>", params.join(", "))
            }
            Value::NativeFunc(_) => write!(f, "<native fn>"),
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            values: HashMap::new(),
        };
        env.define(
            "println".to_string(),
            Value::NativeFunc(|args| {
                for arg in args {
                    print!("{} ", arg);
                }
                println!();
                Value::Unit
            }),
        );
        env
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values.get(name).cloned()
    }
}

pub fn eval_expression(expr: ast::Expr, env: &mut Environment) -> Value {
    match expr {
        ast::Expr::Literal(literal) => match literal {
            ast::Literal::Int(i) => Value::Int(i),
            ast::Literal::Float(f) => Value::Float(f),
            ast::Literal::Bool(b) => Value::Bool(b),
            ast::Literal::Str(s) => Value::Str(s),
        },
        ast::Expr::Variable(name) => match env.get(&name) {
            Some(val) => val,
            None => panic!("Undefined variable: {}", name),
        },
        ast::Expr::Binary(lhs, op, rhs) => {
            let left_val = eval_expression(*lhs, env);
            let right_val = eval_expression(*rhs, env);
            match (left_val, op, right_val) {
                // Integer math.
                (Value::Int(l), BinaryOp::Add, Value::Int(r)) => Value::Int(l + r),
                (Value::Int(l), BinaryOp::Sub, Value::Int(r)) => Value::Int(l - r),
                (Value::Int(l), BinaryOp::Mul, Value::Int(r)) => Value::Int(l * r),
                (Value::Int(l), BinaryOp::Div, Value::Int(r)) => Value::Int(l / r),
                (Value::Int(l), BinaryOp::Lt, Value::Int(r)) => Value::Bool(l < r),
                (Value::Int(l), BinaryOp::Gt, Value::Int(r)) => Value::Bool(l > r),
                (Value::Int(l), BinaryOp::Eq, Value::Int(r)) => Value::Bool(l == r),
                // Mixed math (Int and Float).
                (Value::Int(l), BinaryOp::Add, Value::Float(r)) => Value::Float(l as f64 + r),
                (Value::Float(l), BinaryOp::Add, Value::Int(r)) => Value::Float(l + r as f64),
                (Value::Int(l), BinaryOp::Sub, Value::Float(r)) => Value::Float(l as f64 - r),
                (Value::Float(l), BinaryOp::Sub, Value::Int(r)) => Value::Float(l - r as f64),
                (Value::Int(l), BinaryOp::Mul, Value::Float(r)) => Value::Float(l as f64 * r),
                (Value::Float(l), BinaryOp::Mul, Value::Int(r)) => Value::Float(l * r as f64),
                (Value::Int(l), BinaryOp::Div, Value::Float(r)) => Value::Float(l as f64 / r),
                (Value::Float(l), BinaryOp::Div, Value::Int(r)) => Value::Float(l / r as f64),
                // Float math.
                (Value::Float(l), BinaryOp::Add, Value::Float(r)) => Value::Float(l + r),
                (Value::Float(l), BinaryOp::Sub, Value::Float(r)) => Value::Float(l - r),
                (Value::Float(l), BinaryOp::Mul, Value::Float(r)) => Value::Float(l * r),
                (Value::Float(l), BinaryOp::Div, Value::Float(r)) => Value::Float(l / r),
                (Value::Float(l), BinaryOp::Lt, Value::Float(r)) => Value::Bool(l < r),
                (Value::Float(l), BinaryOp::Gt, Value::Float(r)) => Value::Bool(l > r),
                (Value::Float(l), BinaryOp::Eq, Value::Float(r)) => Value::Bool(l == r),
                // String concatenation.
                (Value::Str(l), BinaryOp::Add, Value::Str(r)) => {
                    let mut new_string = l.clone();
                    new_string.push_str(&r);
                    Value::Str(new_string)
                }
                // Logical operations.
                (Value::Bool(l), BinaryOp::And, Value::Bool(r)) => Value::Bool(l && r),
                (Value::Bool(l), BinaryOp::Or, Value::Bool(r)) => Value::Bool(l || r),
                (l, op, r) => panic!("Type mismatch: {:?} {:?} {:?}", l, op, r),
            }
        }
        ast::Expr::Block(statements, tail) => eval_block(statements, tail, env),
        ast::Expr::If(condition, then_branch, else_branch) => {
            let cond_val = eval_expression(*condition, env);
            if let Value::Bool(true) = cond_val {
                eval_expression(*then_branch, env)
            } else if let Some(else_expr) = else_branch {
                eval_expression(*else_expr, env)
            } else {
                Value::Unit
            }
        }
        ast::Expr::Call(name, args) => {
            let func_val = match env.get(&name) {
                Some(val) => val,
                None => panic!("Undefined function: {}", name),
            };
            let mut arg_values = Vec::new();
            for arg_expr in args {
                arg_values.push(eval_expression(arg_expr, env));
            }
            match func_val {
                Value::Function(params, body) => {
                    if arg_values.len() != params.len() {
                        panic!(
                            "Mismatched arguments: expected {}, got {}",
                            params.len(),
                            arg_values.len()
                        );
                    }
                    // Clone the current environment to support recursion (dynamic scoping).
                    let mut func_env = env.clone();
                    for (param, arg_val) in params.iter().zip(arg_values) {
                        func_env.define(param.clone(), arg_val);
                    }
                    eval_expression(body, &mut func_env)
                }
                Value::NativeFunc(func) => func(arg_values),
                _ => panic!("Can only call functions, not {:?}", func_val),
            }
        }
        ast::Expr::Unary(op, rhs) => {
            let val = eval_expression(*rhs, env);
            match (op, val) {
                (ast::UnaryOp::Neg, Value::Int(i)) => Value::Int(-i),
                (ast::UnaryOp::Neg, Value::Float(f)) => Value::Float(-f),
                (ast::UnaryOp::Not, Value::Bool(b)) => Value::Bool(!b),
                (op, val) => panic!("Cannot apply unary op {:?} to {:?}", op, val),
            }
        }
    }
}

pub fn eval_statement(stmt: ast::Stmt, env: &mut Environment) -> Value {
    match stmt {
        ast::Stmt::Let(name, expr) => {
            let value = eval_expression(expr, env);
            env.define(name, value);
            Value::Unit
        }
        ast::Stmt::Fn(name, params, body) => {
            let func_value = Value::Function(params, body);
            env.define(name, func_value);
            Value::Unit
        }
        ast::Stmt::Expression(expr) => {
            eval_expression(expr, env);
            Value::Unit
        }
        ast::Stmt::ImplicitReturn(expr) => eval_expression(expr, env),
        ast::Stmt::Assign(name, expr) => {
            let value = eval_expression(expr, env);
            if env.get(&name).is_none() {
                panic!("Cannot assign to undefined variable '{}'", name);
            }
            env.define(name, value);
            Value::Unit
        }
        ast::Stmt::While(condition, body) => {
            loop {
                let cond_val = eval_expression(condition.clone(), env);
                match cond_val {
                    Value::Bool(true) => {
                        eval_expression(body.clone(), env);
                    }
                    Value::Bool(false) => {
                        break;
                    }
                    _ => panic!("While loop condition must be a boolean!"),
                }
            }
            Value::Unit
        }
    }
}

fn eval_block(
    statements: Vec<ast::Stmt>,
    tail_expr: Option<Box<ast::Expr>>,
    env: &mut Environment,
) -> Value {
    let mut block_env = env.clone();
    let mut local_vars = HashSet::new();
    for stmt in statements {
        if let ast::Stmt::Let(name, _) = &stmt {
            local_vars.insert(name.clone());
        }
        eval_statement(stmt, &mut block_env);
    }
    // Propagate assignments back to the parent environment.
    for (name, value) in block_env.values.iter() {
        if env.values.contains_key(name) && !local_vars.contains(name) {
            env.define(name.clone(), value.clone());
        }
    }
    if let Some(expr) = tail_expr {
        eval_expression(*expr, &mut block_env)
    } else {
        Value::Unit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn eval_helper(input: &str) -> Value {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut env = Environment::new();
        let mut last_value = Value::Unit;
        for stmt in program {
            last_value = eval_statement(stmt, &mut env);
        }
        last_value
    }

    #[test]
    fn test_scope_mutation() {
        let input = "
            let x = 1;
            if true {
                x = 2;
            }
            x
        ";
        assert_eq!(eval_helper(input), Value::Int(2));
    }

    #[test]
    fn test_while_loop() {
        let input = "
            let x = 5;
            let result = 0;
            while x > 0 {
                result += x;
                x -= 1;
            }
            result
        ";
        assert_eq!(eval_helper(input), Value::Int(15));
    }

    #[test]
    fn test_logical_negation() {
        assert_eq!(eval_helper("!true"), Value::Bool(false));
        assert_eq!(eval_helper("!false"), Value::Bool(true));
        assert_eq!(eval_helper("!!true"), Value::Bool(true));
    }

    #[test]
    fn test_precedence_complex() {
        // 1 + (2 * 3) - (4 / 2) = 1 + 6 - 2 = 5
        assert_eq!(eval_helper("1 + 2 * 3 - 4 / 2"), Value::Int(5));
    }

    #[test]
    fn test_shadowing() {
        let input = "
            let x = 10;
            if true {
                let x = 20; // Shadowing
                x + 1 // 21
            }
            x // Should remain 10
        ";
        assert_eq!(eval_helper(input), Value::Int(10));
    }

    #[test]
    fn test_if_expression_value() {
        let input = "
            let x = if true { 10 } else { 20 };
            x + 5
        ";
        assert_eq!(eval_helper(input), Value::Int(15));
    }

    #[test]
    #[should_panic(expected = "Undefined variable: z")]
    fn test_undefined_variable() {
        eval_helper("let x = 10; x + z");
    }

    #[test]
    fn test_println() {
        assert_eq!(eval_helper(r#"println("hello")"#), Value::Unit);
    }
}
