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
        param_type: Rc<Expr>,
        body: Rc<Expr>,
    },
    App(Rc<Expr>, Rc<Expr>),
    //  Types
    AnyType,
}

impl Expr {
    pub fn is_type(&self) -> bool {
        use Expr::*;
        match self {
            AnyType => true,
            _ => false,
        }
    }

    pub fn is_subtype(&self, other: &Self) -> bool {
        use Expr::*;
        match (self, other) {
            (_, AnyType) => true,
            _ => false,
        }
    }
}

