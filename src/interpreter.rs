use crate::ast;
use crate::ast::BinaryOp;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Unit,
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
        _ => todo!("Implement other expressions"),
    }
}
