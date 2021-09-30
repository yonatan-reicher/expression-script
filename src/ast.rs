use std::collections::hash_map::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug)]
pub enum Expr {
    Var(Ident),
    Func {
        param: Ident,
        body: Rc<Expr>,
    }
}

