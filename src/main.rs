use std::collections::hash_map::HashMap;
use std::rc::Rc;

struct Ident {
    name: String,
}

enum Expr {
    Var(Ident),
    Func {
        param: Ident,
        body: Rc<Expr>,
    }
}

fn main() {
    println!("Hello, world!");
}
