use std::rc::Rc;
use rpds::HashTrieMap;
use crate::ast::{self, Expr};


pub type Context = HashTrieMap<String, Rc<Expr>>;

pub fn reduce(expr: &Expr, context: &Context) -> Option<Rc<Expr>> {
    match expr {
        Expr::Var(ident) => context.get(&ident.name).cloned(),
        Expr::Func { param, body } => {
            //  Remove the parameter from the context for cases like
            //  (x -> x -> x)
            let bodyContext = context.remove(&param.name);
            //  Reduce the body
            reduce(body.as_ref(), &bodyContext)
            .map(|body| Rc::new(Expr::Func { param: param.clone(), body }))
        },
    }
}
