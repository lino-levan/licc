pub enum Token {
    Identifier(String),
    Directive(String),

    // Literals
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),

    // Keywords
    Return,

    // Random tokens
    LeftParanthesis,
    RightParanthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
}

pub struct Tokenizer {
    content: String,
    index: usize,
}

impl Tokenizer {
    pub fn new(content: String) -> Self {
        Tokenizer { content, index: 0 }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.content.chars().skip(self.index);
        let start = chars.next()?;
        self.index += 1;

        match start {
            'a'..='z' | 'A'..='Z' | '_' | '#' => {
                let mut identifier = String::new();
                identifier.push(start);

                while let Some(c) = chars.next() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                    } else {
                        break;
                    }
                }

                self.index += identifier.len() - 1;
                match identifier.as_str() {
                    "return" => Some(Token::Return),
                    _ if identifier.starts_with('#') => Some(Token::Directive(
                        identifier.trim_start_matches("#").to_string(),
                    )),
                    _ => Some(Token::Identifier(identifier)),
                }
            }
            '"' => {
                let mut string = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    string.push(c);
                }

                self.index += string.len() + 1;
                Some(Token::StringLiteral(string))
            }
            '0'..='9' => {
                let mut number = String::new();
                number.push(start);

                while let Some(c) = chars.next() {
                    if c.is_numeric() || c == '.' {
                        number.push(c);
                    } else {
                        break;
                    }
                }

                self.index += number.len() - 1;
                if number.contains('.') {
                    Some(Token::FloatLiteral(number.parse().unwrap()))
                } else {
                    Some(Token::IntegerLiteral(number.parse().unwrap()))
                }
            }
            '(' => Some(Token::LeftParanthesis),
            ')' => Some(Token::RightParanthesis),
            '[' => Some(Token::LeftBracket),
            ']' => Some(Token::RightBracket),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ';' => Some(Token::Semicolon),
            ' ' | '\n' | '\t' => {
                self.index += 1;
                self.next()
            }
            _ => None,
        }
    }
}
