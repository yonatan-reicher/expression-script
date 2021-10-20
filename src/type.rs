use std::rc::Rc;
use rpds::HashTrieMap;
use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Type {
    Any,
}

pub type VariableTypes<'a> = HashTrieMap<&'a str, Rc<Type>>;

impl Type {
    pub fn from_expr(expr: &Expr) -> Option<Self> {
        match expr {
            Expr::AnyType => Some(Self::Any),
            Expr::Var(_) | Expr::App(_, _) | Expr::Func { .. } => None,
        }
    }

    pub fn subtype_of(&self, other: &Type) -> bool {
        match (self, other) {
            _ => true,
        }
    }

    pub fn of_expr(expr: &Expr, con: &VariableTypes) -> Option<Self> {
        match expr {
            Expr::Var(var) => con.get(var.name.as_str()).map(Rc::as_ref).cloned(),
            Expr::Func { .. } => Some(Type::Any),
            Expr::App(left, right) => {
                let left_type = Type::of_expr(left, con)?;
                Some(Type::Any)
            },
            Expr::AnyType => Some(Type::Any),
        }
    }
}
