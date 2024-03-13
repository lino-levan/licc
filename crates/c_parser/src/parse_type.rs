use std::iter::Peekable;
use c_ast::Type;

use crate::tokenizer::{Token, Tokenizer};

pub fn parse_type(tokens: &mut Peekable<Tokenizer>) -> Type {
    match tokens.next() {
        Some(Token::Identifier(identifier)) => {
            if identifier == "int" {
                Type::CType(c_ast::CType::Int)
            } else {
                panic!("Not a valid type 3")
            }
        }
        _ => panic!("Not a valid type")
    }
}
