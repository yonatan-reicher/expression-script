use std::rc::Rc;
use nom::{
    IResult,
    character::complete::{alphanumeric1, space0, char},
    bytes::complete::tag,
    combinator::map,
    multi::many_m_n,
    branch::alt,
    sequence::tuple,
};
use crate::ast;

fn ident(input: &str) -> IResult<&str, ast::Ident> {
    map(alphanumeric1, |name: &str| ast::Ident { name: name.into() }) (input)
}

fn func(input: &str) -> IResult<&str, ast::Expr> {
    map(
        tuple((ident, space0, tag("->"), space0, expr)),
        |(param, _, _, _, body)|
            ast::Expr::Func { param, body: Rc::new(body) },
    ) (input)
}

fn variable(input: &str) -> IResult<&str, ast::Expr> {
    map(ident, ast::Expr::Var) (input)
}

fn app(input: &str) -> IResult<&str, ast::Expr> {
    map(many_m_n(2, usize::MAX, tuple((atom, space0))), |vec| {
        vec.into_iter()
        .map(|(x, _)| x)
        .reduce(|left, right| ast::Expr::App(Rc::new(left), Rc::new(right)))
        .unwrap()
    }) (input)
}

fn atom(input: &str) -> IResult<&str, ast::Expr> {
    alt((
        variable,
        map(tuple((char('('), expr, char(')'))), |(_, x, _)| x),
    )) (input)
}

fn expr(input: &str) -> IResult<&str, ast::Expr> {
    alt((
        func,
        app,
        atom,
    )) (input)
}

pub fn parse(input: &str) -> Result<ast::Expr, ()> {
    match expr(input) {
        Ok(("", ast)) => Ok(ast),
        Ok(_) => Err(()),
        Err(_) => Err(()),
    }
}
