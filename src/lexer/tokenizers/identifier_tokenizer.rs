use regex::Regex;

use crate::lexer::{TokenSpan, Token};

use super::{TokenWrapper, Tokenizer, Position, consume_regex};

pub struct IdentifierTokenizer {
    regex: Regex,
}

impl IdentifierTokenizer {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"^([a-zA-Z][a-zA-Z0-9_$]*)").unwrap(),
        }
    }
}

impl Tokenizer for IdentifierTokenizer {
    fn tokenize(&self, contents: &str, position: &Position) -> Option<TokenWrapper> {
        let view = &contents[position.offset..];

        let (text, length) = consume_regex(view, &self.regex)?;

        Some(TokenWrapper {
            token: Token::Identifier(text),
            span: TokenSpan::new(position.line, position.offset, length),
        })
    }
}
