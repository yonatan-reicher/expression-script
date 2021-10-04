use std::collections::hash_map::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(Ident),
    Func {
        param: Ident,
        body: Rc<Expr>,
    },
    App(Rc<Expr>, Rc<Expr>),
}

