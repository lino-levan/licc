use std::iter::Peekable;
use c_ast::Expression;

use crate::tokenizer::{Token, Tokenizer};

pub fn parse_expression(tokens: &mut Peekable<Tokenizer>) -> Expression {
    match tokens.next() {
        Some(Token::IntegerLiteral(int)) => Expression::IntegerLiteral(int),
        _ => panic!("waaa")
    }
}
