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
    FuncType(Rc<Expr>, Rc<Expr>),
}

// Implement conversions to Ident
impl From<&str> for Ident {
    fn from(name: &str) -> Self {
        Ident {
            name: name.to_string(),
        }
    }
}

