use std::iter::Peekable;
use c_ast::TopLevelStatement;

use crate::{parse_function_declaration::parse_function_declaration, tokenizer::Tokenizer};

pub fn parse_top_level_statement(tokens: &mut Peekable<Tokenizer>) -> TopLevelStatement {
    TopLevelStatement::FunctionDeclaration(parse_function_declaration(tokens))
}
