
pub enum TokenKind {
    Word(String),
}

pub struct Span(usize, usize);

pub struct Token {
    Kind: TokenKind,
    span: Span,
}

pub struct TokenStream<'source> {
    source: &'source str,
    index: usize,
}

