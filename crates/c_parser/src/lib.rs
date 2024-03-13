use c_ast::Module;

mod tokenizer;
mod parse_top_level_statement;
mod parse_function_declaration;
mod parse_statement;
mod parse_type;
mod parse_expression;

use tokenizer::Tokenizer;
use parse_top_level_statement::parse_top_level_statement;

pub fn parse(content: String) -> Module {
    let mut tokens = Tokenizer::new(content).peekable();
    let mut statements = Vec::new();

    while tokens.peek().is_some() {
        statements.push(parse_top_level_statement(&mut tokens));
    }

    Module { statements }
}
