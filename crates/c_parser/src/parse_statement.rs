use std::iter::Peekable;
use c_ast::Statement;

use crate::{parse_expression::parse_expression, tokenizer::{Token, Tokenizer}};

pub fn parse_statement(tokens: &mut Peekable<Tokenizer>) -> Statement {
    match tokens.next() {
        Some(Token::Return) => {
            let expression = parse_expression(tokens);
            match tokens.next() {
                Some(Token::Semicolon) => (),
                _ => panic!("what the frick man")
            }
            Statement::Return(expression)
        }
        _ => panic!("AAAA")
    }
}
