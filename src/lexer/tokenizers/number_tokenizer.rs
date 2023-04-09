use regex::Regex;

use crate::lexer::{Token, TokenSpan};

use super::{consume_regex, Position, TokenWrapper, Tokenizer};

pub struct NumberTokenizer {
    regex: Regex,
}

impl NumberTokenizer {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"^([0-9]+\.?[0-9]*)").unwrap(),
        }
    }
}

impl Tokenizer for NumberTokenizer {
    fn tokenize(&self, contents: &str, position: &Position) -> Option<TokenWrapper> {
        let view = &contents[position.offset..];
        let (text, length) = consume_regex(view, &self.regex)?;

        Some(TokenWrapper {
            token: Token::NumberLiteral(text.parse::<f64>().unwrap()),
            span: TokenSpan::new(position.line, position.offset, length),
        })
    }
}
