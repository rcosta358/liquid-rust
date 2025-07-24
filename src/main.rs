lalrpop_util::lalrpop_mod!(pub grammar);

use lalrpop_util::lalrpop_mod;
use lalrpop_util::ParseError;
use logos::Logos;
use crate::grammar::ExprParser;
use crate::{ast::Expr, lexer::Token};

mod lexer;
mod ast;

fn main() {
    let input = "1 + 2.5 * (3 - 4)";
    match parse_expr(input) {
        Ok(expr) => println!("{:#?}", expr),
        Err(e) => eprintln!("Parse error: {:?}", e),
    }
}

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