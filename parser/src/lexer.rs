use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error=LexicalError)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token(">=")]
    Ge,

    #[token("<=")]
    Le,

    #[token(">")]
    Gt,

    #[token("<")]
    Lt,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("%")]
    Mod,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("?")]
    Question,

    #[token(":")]
    Colon,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("_")]
    Id,

    #[regex(r"[0-9]+", |lex| lex.slice().parse().map_err(|_| LexicalError::InvalidInteger))]
    Int(i64),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger,
    #[default]
    UnrecognizedToken,
}