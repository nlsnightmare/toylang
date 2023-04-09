use crate::lexer::{TokenSpan, Token};

use super::{Position, TokenWrapper, Tokenizer};

pub struct StringTokenizer;

impl StringTokenizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tokenizer for StringTokenizer {
    fn tokenize(&self, contents: &str, position: &Position) -> Option<TokenWrapper> {
        let view = &contents[position.offset..];

        if !view.starts_with('"') {
            return None;
        }

        // parse string
        let start = 1;
        let mut len = 0;
        for c in view[start..].chars() {
            if c == '"' {
                break;
            }
            len += 1
        }

        let end = start + len;
        let literal = view[start..end].to_owned();

        return Some(TokenWrapper {
            span: TokenSpan::new(position.line, start, literal.len() + 2),
            token: Token::StringLiteral(literal),
        });
    }
}
