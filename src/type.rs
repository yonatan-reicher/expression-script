use std::rc::Rc;
use rpds::HashTrieMap;
use crate::ast::*;
use crate::reduce;

#[derive(Debug, Clone)]
pub enum Type {
    Any,
    Func(Rc<Type>, Rc<Type>),
}

pub type VariableTypes<'a> = HashTrieMap<&'a str, Rc<Type>>;

impl Type {
    pub fn from_expr(expr: &Expr) -> Option<Self> {
        match expr {
            Expr::AnyType => Some(Self::Any),
            Expr::FuncType(left, right) => {
                let left = Rc::new(Type::from_expr(&left)?);
                let right = Rc::new(Type::from_expr(&right)?);
                Some(Type::Func(left, right))
            }
            Expr::Var(_) | Expr::App(_, _) | Expr::Func { .. } => None,
        }
    }

    pub fn is_subtype_of(&self, other: &Type) -> bool {
        match (self, other) {
            _ => true,
        }
    }

    pub fn of_expr(expr: &Expr, con: &VariableTypes) -> Option<Rc<Self>> {
        match expr {
            Expr::Var(var) => con.get(var.name.as_str()).cloned(),
            Expr::Func { param, param_type: param_type_expr, body } => {
                let param_type_expr = reduce::reduce_or_ret(param_type_expr.clone());
                let param_type = Rc::new(Self::from_expr(&param_type_expr)?);
                let body_context = con.insert(&param.name, param_type.clone());
                let body_type = Self::of_expr(&body, &body_context)?;
                Some(Rc::new(Type::Func(param_type, body_type)))
            }
            Expr::App(left, right) => {
                let left_type = Type::of_expr(left, con)?;
                let right_type = Type::of_expr(right, con)?;
                match left_type.as_ref() {
                    Type::Func(input_type, output_type) => {
                        if right_type.is_subtype_of(&input_type) {
                            Some(output_type.clone())
                        } else {
                            None
                        }
                    },
                    _ => None,
                }
            },
            Expr::AnyType => Some(Rc::new(Type::Any)),
            Expr::FuncType(_,_) => Some(Rc::new(Type::Any)),
        }
    }
}
