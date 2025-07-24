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

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[regex(r"[0-9]+", priority = 2, callback = |lex| lex.slice().parse().map_err(|_| LexicalError::InvalidInteger))]
    Int(i64),
    
    #[regex(r"[0-9]+(\.[0-9]+)?", priority = 1, callback = |lex| lex.slice().parse().map_err(|_| LexicalError::InvalidFloat))]
    Float(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Id(String),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger,
    InvalidFloat,
    #[default]
    UnrecognizedToken,
}