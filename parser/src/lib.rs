lalrpop_util::lalrpop_mod!(pub grammar);

use lalrpop_util::lalrpop_mod;
use lalrpop_util::ParseError;
use logos::Logos;
use crate::ast::Expr;
use crate::grammar::ExprParser;
use crate::lexer::Token;

mod lexer;
pub mod ast;

pub fn parse_expr(input: &str) -> Result<Expr, ParseError<usize, Token, String>> {
    let lexer = Token::lexer(input);
    let tokens = lexer.spanned().map(|(tok, span)| {
        match tok {
            Ok(token) => Ok((span.start, token, span.end)),
            Err(e) => Err(format!("{:?}", e)),
        }
    });
    ExprParser::new().parse(tokens)
}