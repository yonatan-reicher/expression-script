//! Module for walking the AST and reducing it to a minimal value.
//!
//! This module contains the `reduce` function which takes an AST and reduces
//! it to a single value.

use std::rc::Rc;
use crate::ast::Expr;
use crate::r#type::*;


/// Reduce an AST.
///
/// This function takes an AST and reduces it as much as possible.
/// For example, it will reduce `(x: any -> x) a` to `a`.
/// Notice: it does not execute the AST - Only reduces it.
///
/// If the AST cannot be reduced, None will be returned.
pub fn reduce(expr: &Expr) -> Option<Rc<Expr>> {
    match expr {
        Expr::Var(_) => None,
        Expr::AnyType => None,
        Expr::Func { param, param_type, body } => {
            let mut reduced_flag = false;

            //  Reduce the parameter's type
            let param_type = reduce_with_flag(param_type.clone(), &mut reduced_flag);
            
            // Reduce the body
            let body = reduce_with_flag(body.clone(), &mut reduced_flag);

            // If both weren't reduced, return None
            if !reduced_flag {
                return None;
            }

            // Return the reduced expression
            Some(Expr::Func {
                param: param.clone(),
                param_type: param_type,
                body,
            }.into())
        },
        Expr::App(func, arg) => {
            let mut reduced_flag = false;

            // Reduce the function
            let func = reduce_with_flag(func.clone(), &mut reduced_flag);

            // Reduce the argument
            let arg = reduce_with_flag(arg.clone(), &mut reduced_flag);

            // try applying the function to the argument
            match func.as_ref() {
                Expr::Func { param, param_type, body }
                if Type::from_expr(param_type).is_some() => {
                    Some(substitute(body.clone(), &param.name, arg.clone()))
                },
                _ => {
                    // If both weren't reduced, return None
                    if !reduced_flag {
                        return None;
                    }

                    // Return the reduced expression
                    Some(Expr::App(func, arg).into())
                },
            }
        },
        Expr::FuncType(left, right) => {
            let reduced_left = reduce(&left);
            let reduced_right = reduce(&right);
            match (reduced_left, reduced_right) {
                (None, None) => None,
                (reduced_left, reduced_right) => {
                    let left = reduced_left.unwrap_or(left.clone());
                    let right = reduced_right.unwrap_or(right.clone());
                    Some(Rc::new(Expr::FuncType(left, right)))
                }
            }
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
        Expr::AnyType => expr,
        Expr::Func { param, param_type, body } => {
            let body = recurse(body.clone());
            let param = param.clone();
            let param_type = recurse(param_type.clone());
            Rc::new(Expr::Func { param, param_type, body })
        },
        Expr::App(left, right) => {
            let left = recurse(left.clone());
            let right = recurse(right.clone());
            Rc::new(Expr::App(left, right))
        },
        Expr::FuncType(left, right) => {
            let left = recurse(left.clone());
            let right = recurse(right.clone());
            Rc::new(Expr::FuncType(left, right))
        }
    }
}

pub fn reduce_or_ret(expr: Rc<Expr>) -> Rc<Expr> { reduce(expr.as_ref()).unwrap_or(expr) }

/// Reduce an AST or return it as is. If was reduced, set the flag to true.
pub fn reduce_with_flag(expr: Rc<Expr>, reduced_flag: &mut bool) -> Rc<Expr> {
    let reduced = reduce(expr.as_ref());
    *reduced_flag |= reduced.is_some();
    reduced.unwrap_or(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use crate::ast::*;
    use crate::ast::Expr::*;

    fn r<T>(v: T) -> Rc<T> { Rc::new(v) }


    #[test]
    fn cannot_reduce_any_type() {
        assert_eq!(reduce(&AnyType), None);
    }

    #[test]
    fn cannot_reduce_var() {
        assert_eq!(reduce(&Var("x".into())), None);
    }

    #[test]
    fn cannot_reduce_func_type() {
        assert_eq!(reduce(&FuncType(AnyType.into(), AnyType.into())), None);
    }

    #[test]
    fn reduce_application() {
        // (x: any -> x) a ===> a
        assert_eq!(
            reduce(&App(
                Func {
                    param: "x".into(),
                    param_type: AnyType.into(),
                    body: Var("x".into()).into(),
                }.into(),
                Var("x".into()).into(),
            )),
            Some(Var("x".into()).into())
        );
    }

    #[test]
    fn reduce_nested_application() {
        // (x: any -> (y: any -> x)) a b ===> a
        assert_eq!(
            reduce(&App(
                App(
                    Func {
                        param: "x".into(),
                        param_type: AnyType.into(),
                        body: Func {
                            param: "y".into(),
                            param_type: AnyType.into(),
                            body: Var("x".into()).into(),
                        }.into(),
                    }.into(),
                    Var("a".into()).into(),
                ).into(),
                Var("b".into()).into(),
            )),
            Some(Var("a".into()).into())
        );
    }

    #[test]
    fn reduce_partial_application() {
        // (x: any -> y: any -> x) a ===> (y: any -> a)
        assert_eq!(
            reduce(&App(
                Func {
                    param: "x".into(),
                    param_type: AnyType.into(),
                    body: Func {
                        param: "y".into(),
                        param_type: AnyType.into(),
                        body: Var("x".into()).into(),
                    }.into(),
                }.into(),
                Var("a".into()).into(),
            )),
            Some(Func {
                param: "y".into(),
                param_type: AnyType.into(),
                body: Var("a".into()).into(),
            }.into())
        );
    }

    #[test]
    fn reduce_nested_application_in_type() {
        // (x: ((t: any -> any) a) -> x) b ===> b
        assert_eq!(
            reduce(&App(
                Func {
                    param: "x".into(),
                    param_type: App(
                        Func {
                            param: "t".into(),
                            param_type: AnyType.into(),
                            body: AnyType.into(),
                        }.into(),
                        Var("a".into()).into(),
                    ).into(),
                    body: Var("x".into()).into(),
                }.into(),
                Var("b".into()).into(),
            )),
            Some(Var("b".into()).into())
        );
    }
}
