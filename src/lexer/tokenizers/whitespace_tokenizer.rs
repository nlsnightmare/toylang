use regex::Regex;

use crate::lexer::{Token, TokenSpan, TokenWrapper};

use super::{consume_regex, Position, Tokenizer};

pub struct WhitespaceTokenizer {
    regex: Regex,
}

impl WhitespaceTokenizer {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"^(\s+)").unwrap(),
        }
    }
}

impl Tokenizer for WhitespaceTokenizer {
    fn tokenize(&self, contents: &str, position: &Position) -> Option<TokenWrapper> {
        let view = &contents[position.offset..];

        let (_, length) = consume_regex(view, &self.regex)?;

        Some(TokenWrapper {
            token: Token::Whitespace,
            span: TokenSpan::new(position.line, position.offset, length),
        })
    }
}
