use std::rc::Rc;
use rpds::HashTrieMap;
use crate::ast::{self, Expr};


pub fn reduce(expr: &Expr) -> Option<Rc<Expr>> {
    let mut ret = None;
    loop {
        match reduce1(expr) {
            Some(new_expr) => ret = Some(new_expr),
            None => break ret,
        }
    }
}

pub fn substitute(expr: Rc<Expr>, name: &str, value: Rc<Expr>) -> Rc<Expr> {
    let recurse = |expr: Rc<Expr>| {
        substitute(expr.clone(), name, value.clone())
    };
    match expr.as_ref() {
        //  cases where an identifier equals name
        Expr::Var(ident) if &ident.name == name => value,
        Expr::Func { param, .. } if &param.name == name => expr,
        //  regular case
        Expr::Var(_) => expr,
        Expr::Func { param, body } => {
            let body = recurse(body.clone());
            let param = param.clone();
            Rc::new(Expr::Func { param, body })
        },
        Expr::App(left, right) => {
            let left = recurse(left.clone());
            let right = recurse(right.clone());
            Rc::new(Expr::App(left, right))
        },
    }
}

pub fn reduce1(expr: &Expr) -> Option<Rc<Expr>> {
    match expr {
        Expr::Var(_) => None,
        Expr::Func { param, body } => {
            //  Reduce the body
            reduce(body.as_ref())
            .map(|body| Rc::new(Expr::Func { param: param.clone(), body }))
        },
        Expr::App(func, arg) => {
            match func.as_ref() {
                Expr::Func { param, body } => {
                    Some(substitute(body.clone(), &param.name, arg.clone()))
                },
                _ => {
                    let func = reduce(func)?;
                    let arg = reduce(arg)?;
                    Some(Rc::new(Expr::App(func, arg)))
                },
            }
        },
    }
}

pub fn reduce1_or_ret(expr: Rc<Expr>) -> Rc<Expr> {
    reduce(expr.as_ref()).unwrap_or(expr)
}

