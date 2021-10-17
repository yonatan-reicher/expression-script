use std::rc::Rc;
use crate::ast::Expr;


pub fn reduce(expr: &Expr) -> Option<Rc<Expr>> {
    let mut ret: Rc<Expr> = reduce1(expr)?;
    loop {
        match reduce1(ret.as_ref()) {
            Some(new_expr) => ret = new_expr.clone(),
            None => break Some(ret),
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
    }
}

pub fn reduce1(expr: &Expr) -> Option<Rc<Expr>> {
    match expr {
        Expr::Var(_) => None,
        Expr::AnyType => None,
        Expr::Func { param, param_type, body } => {
            //  Reduce the parameter's type
            reduce(param_type)
            .map(|param_type| {
                let param = param.clone();
                let body = body.clone();
                Rc::new(Expr::Func { param_type, param, body })
            })
            .or_else(|| {
                //  Reduce the body
                reduce(body.as_ref())
                .map(|body| {
                    let param = param.clone();
                    let param_type = param_type.clone();
                    Rc::new(Expr::Func { param, param_type, body })
                })
            })
        },
        Expr::App(func, arg) => {
            match func.as_ref() {
                Expr::Func { param, param_type, body } if param_type.is_type() => {
                    Some(substitute(body.clone(), &param.name, arg.clone()))
                },
                _ => {
                    let reduced_func = reduce(func.as_ref());
                    let reduced_arg = reduce(arg.as_ref());
                    match (reduced_func, reduced_arg) {
                        (None, None) => None,
                        (reduced_func, reduced_arg) => {
                            let func = reduced_func.unwrap_or(func.clone());
                            let arg = reduced_arg.unwrap_or(arg.clone());
                            Some(Rc::new(Expr::App(func, arg)))
                        }
                    }
                },
            }
        },
    }
}

pub fn reduce1_or_ret(expr: Rc<Expr>) -> Rc<Expr> {
    reduce(expr.as_ref()).unwrap_or(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use crate::ast::*;
    use crate::ast::Expr::*;

    fn r<T>(v: T) -> Rc<T> { Rc::new(v) }


    #[test]
    fn reduce_application() {
        let x = Ident { name: "x".into() };
        let y = Ident { name: "y".into() };
        let id = Func { param: y.clone(), param_type: r(AnyType), body: r(Var(y.clone())) };
        let f = Func { param: x.clone(), param_type: r(AnyType), body: r(App(r(Var(x.clone())), r(Var(x.clone())))) };
        let expr = App(r(f.clone()), r(id.clone()));
        let reduced: Rc<Expr> = reduce(&expr).unwrap();
        assert_eq!(reduced.as_ref(), &Expr::Func {
            param: y.clone(),
            param_type: r(AnyType),
            body: r(Var(y.clone())),
        });
    }
}
