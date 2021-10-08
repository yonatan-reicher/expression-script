use std::rc::Rc;
use nom::{
    IResult,
    character::is_alphanumeric,
    character::complete::{self, satisfy, space0, char},
    bytes::complete::tag,
    combinator::map,
    multi::{many_m_n, many1},
    branch::alt,
    sequence::tuple,
};
use crate::ast;

fn ident_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| is_alphanumeric(c as u8) || c == '_') (input)
}

fn ident(input: &str) -> IResult<&str, ast::Ident> {
    map(
        many1(ident_char),
        |name_chars| ast::Ident { name: name_chars.iter().collect() }
    ) (input)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::*;

    #[test]
    fn parse_variable() {
        let code: &str = "variable";
        println!("{:?}", parse(code));
        assert!(matches!(
            parse(code),
            Ok(Expr::Var(ref var)) if var.name == "variable"
        ));
    }

    #[test]
    fn parse_variable_with_underscore() {
        let code: &str = "my_variable";
        println!("{:?}", parse(code));
        assert!(matches!(
            parse(code),
            Ok(Expr::Var(ref var)) if var.name == "my_variable"
        ));
    }
}
