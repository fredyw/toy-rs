use crate::ast;
use crate::ast::BinaryOp;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Unit,
    Function(Vec<String>, ast::Expr),
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
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
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
                (l, op, r) => panic!("Type mismatch: {:?} {:?} {:?}", l, op, r),
            }
        }
        ast::Expr::Block(statements, tail) => eval_block(statements, tail, env),
        ast::Expr::If(condition, then_branch, else_branch) => {
            let cond_val = eval_expression(*condition, env);
            if let Value::Bool(true) = cond_val {
                eval_expression(*then_branch, env)
            } else {
                if let Some(else_expr) = else_branch {
                    eval_expression(*else_expr, env)
                } else {
                    Value::Unit
                }
            }
        }
        ast::Expr::Call(name, args) => {
            let func_val = match env.get(&name) {
                Some(val) => val,
                None => panic!("Undefined function: {}", name),
            };
            match func_val {
                Value::Function(params, body) => {
                    if args.len() != params.len() {
                        panic!(
                            "Mismatched arguments: expected {}, got {}",
                            params.len(),
                            args.len()
                        );
                    }
                    let mut func_env = Environment::new();
                    for (param, arg_expr) in params.iter().zip(args) {
                        let arg_val = eval_expression(arg_expr, env);
                        func_env.define(param.clone(), arg_val);
                    }
                    eval_expression(body, &mut func_env)
                }
                _ => panic!("Can only call functions, not {:?}", func_val),
            }
        }
        _ => todo!("Implement other expressions"),
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
    }
}

fn eval_block(
    statements: Vec<ast::Stmt>,
    tail_expr: Option<Box<ast::Expr>>,
    env: &mut Environment,
) -> Value {
    let mut block_env = env.clone();
    for stmt in statements {
        eval_statement(stmt, &mut block_env);
    }
    if let Some(expr) = tail_expr {
        eval_expression(*expr, &mut block_env)
    } else {
        Value::Unit
    }
}
