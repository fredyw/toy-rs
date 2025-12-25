use crate::ast;
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
            todo!("Implement math evaluation");
        }
        _ => todo!("Implement other expressions"),
    }
}
