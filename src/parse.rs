use std::rc::Rc;
use nom::{
    IResult,
    character::complete::{alphanumeric1, space0},
    bytes::complete::tag,
    combinator::{
        map,
    },
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

fn expr(input: &str) -> IResult<&str, ast::Expr> {
    alt((
        func,
        variable,
    )) (input)
}

pub fn parse(input: &str) -> Result<ast::Expr, ()> {
    match expr(input) {
        Ok((inp, ast)) => Ok(ast),
        Err(_) => Err(()),
    }
}
