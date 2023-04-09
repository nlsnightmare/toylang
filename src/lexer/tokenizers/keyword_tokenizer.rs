use crate::lexer::{TokenSpan, Token, Keyword};

use super::{Position, TokenWrapper, Tokenizer};

pub struct KeywordTokenizer<'a> {
    keywords: Vec<&'a str>,
}

impl KeywordTokenizer<'_> {
    pub fn new() -> Self {
        Self {
            keywords: Keyword::all(),
        }
    }
}

impl Tokenizer for KeywordTokenizer<'_> {
    fn tokenize(&self, contents: &str, position: &Position) -> Option<TokenWrapper> {
        let view = &contents[position.offset..];

        for keyword in self.keywords.iter() {
            if !view.starts_with(keyword) {
                continue;
            }

            let len = keyword.len();
            let keyword = Keyword::from_string(&keyword);
            let token = match keyword {
                Keyword::True => Token::BooleanLiteral(true),
                Keyword::False => Token::BooleanLiteral(false),
                _ => Token::Keyword(keyword),
            };

            return Some(TokenWrapper {
                token,
                span: TokenSpan::new(position.line, position.offset, len),
            });
        }

        return None;
    }
}
