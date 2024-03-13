use std::iter::Peekable;
use c_ast::FunctionDeclaration;

use crate::parse_statement::parse_statement;
use crate::parse_type::parse_type;
use crate::tokenizer::Tokenizer;
use crate::tokenizer::Token;

pub fn parse_function_declaration(tokens: &mut Peekable<Tokenizer>) -> FunctionDeclaration {
    let return_type = parse_type(tokens);
    let name = match tokens.next().unwrap() {
        Token::Identifier(string) => string,
        _ => panic!(":(")
    };

    match tokens.next() {
        Some(Token::LeftParanthesis) => (),
        _ => panic!("what the frick man")
    }

    let parameters = Vec::new();
    // TODO(lino-levan): Parse paramters

    match tokens.next() {
        Some(Token::RightParanthesis) => (),
        _ => panic!("what the frick man")
    }

    match tokens.next() {
        Some(Token::LeftBrace) => (),
        bingus@_ => panic!("what the frick man {:?}", bingus)
    }

    let mut body = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::RightBrace) => {
                tokens.next();
                break;
            },
            _ => ()
        }

        body.push(parse_statement(tokens));
    }

    FunctionDeclaration { name, parameters, return_type, body }
}
