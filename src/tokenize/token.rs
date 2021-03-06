use tokenize::span::Span;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token<'a> {
    Capitalize(Span<'a>),
    Char(Span<'a>),
    Comment(Span<'a>),
    Float(Span<'a>),
    Int(Span<'a>),
    String(Span<'a>),
    Symbol(Span<'a>),
    Identifier(Span<'a>),
    Keyword(Span<'a>),
}

impl<'a> Token<'a> {
    pub fn span(&self) -> Span<'a> {
        let s = match self {
            Token::Capitalize(s) => s,
            Token::Char(s) => s,
            Token::Comment(s) => s,
            Token::Float(s) => s,
            Token::Identifier(s) => s,
            Token::Int(s) => s,
            Token::Keyword(s) => s,
            Token::String(s) => s,
            Token::Symbol(s) => s,
        };

        *s
    }
}