use crate::ast::*;
use std::fmt::{Display, Formatter, Result};


impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> Result { self.name.fmt(f) }
}

fn is_atom(expr: &Expr) -> bool {
    match expr {
        Expr::Var(_) | Expr::AnyType => true,
        Expr::Func { .. } | Expr::App(_, _) => false,
    }
}

fn print_as_atom(expr: &Expr, f: &mut Formatter) -> Result {
    if is_atom(expr) { expr.fmt(f) } else {
        '('.fmt(f)?;
        expr.fmt(f)?;
        ')'.fmt(f)
    }
}

fn print_as_expr(expr: &Expr, f: &mut Formatter) -> Result { expr.fmt(f) }

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Expr::Var(ident) => ident.fmt(f),
            Expr::AnyType => "any".fmt(f),
            Expr::Func { param, param_type, body } => {
                param.fmt(f)?;
                ": ".fmt(f)?;
                print_as_atom(param_type, f)?;
                " -> ".fmt(f)?;
                print_as_expr(body, f)
            },
            Expr::App(left, right) => {
                print_as_atom(left, f)?;
                ' '.fmt(f)?;
                print_as_atom(right, f)
            },
        }
    }
}

