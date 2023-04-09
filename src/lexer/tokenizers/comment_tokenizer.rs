use regex::Regex;

use crate::lexer::{Token, TokenSpan};

use super::{TokenWrapper, Tokenizer, Position, consume_regex};

pub struct CommentTokenizer {
    regex: Regex,
}

impl CommentTokenizer {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(r"^(#.*$)").unwrap(),
        }
    }
}

impl Tokenizer for CommentTokenizer {
    fn tokenize(&self, contents: &str, position: &Position) -> Option<TokenWrapper> {
        let view = &contents[position.offset..];
        let (text, length) = consume_regex(view, &self.regex)?;

        Some(TokenWrapper {
            token: Token::Comment(text),
            span: TokenSpan::new(position.line, position.offset, length),
        })
    }
}
