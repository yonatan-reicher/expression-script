use std::collections::hash_map::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Var(Ident),
    Func {
        param: Ident,
        body: Rc<Expr>,
    }
}

