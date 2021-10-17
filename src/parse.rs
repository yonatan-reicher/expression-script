use std::rc::Rc;
use nom::{
    IResult,
    character::is_alphanumeric,
    character::complete::{satisfy, space0, char},
    bytes::complete::tag,
    combinator::map,
    multi::{many_m_n, many1},
    branch::alt,
    sequence::tuple,
};
use crate::ast::{self, Expr};

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
        tuple((ident, space0, tag(":"), space0, tag("any"), space0, tag("->"), space0, expr)),
        |(param, _, _, _, _, _, _, _, body)|
            ast::Expr::Func { param, param_type: Rc::new(Expr::AnyType), body: Rc::new(body) },
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

    /// test that parsing identifier from (name + after) returns correctly
    fn ident_helper(name: &str, after: &str) {
        let source = format!("{}{}", name, after);
        println!("Parsing identifier from '{}'", &source);
        let (rest, identifier) =
            ident(&source)
            .expect("Parsing identifier");
        assert_eq!(rest, after);
        assert_eq!(identifier.name, name);
    }

    #[test]
    fn ident_simple() { ident_helper("variable", ""); }
    #[test]
    fn ident_with_underscores() { ident_helper("whats_up", ""); }
    #[test]
    fn ident_with_digits() { ident_helper("2good4u", ""); }
    #[test]
    fn ident_with_space_afterwards() { ident_helper("my_variable", " "); }
    #[test]
    fn ident_with_dash_afterwards() { ident_helper("daniel", "->"); }
 
    fn func_helper(first: &str, second: &str, third: &str, fourth: &str, fifth: &str, after: &str) {
        let source = format!("{}{}{}{}{}{}", first, second, third, fourth, fifth, after);
        println!("Parsing function from '{}'", source);
        let (rest, func) = func(&source).expect("Parse func");
        assert_eq!(rest, after);
        match func {
            Expr::Func { param, param_type, body } => {
                assert_eq!(param.name, first);
            },
            _ => panic!("func did not return a function"),
        }
    }

    #[test]
    fn func_simple() { func_helper("arg", ": ", "any", " -> ", "body with arg", ""); }
}
