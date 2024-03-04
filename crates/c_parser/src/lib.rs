mod tokenizer;
use tokenizer::Tokenizer;

pub fn parse(content: String) {
    let tokens = Tokenizer::new(content);
}
