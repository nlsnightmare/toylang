use crate::lexer::{TokenSpan, Token};

use super::{Position, TokenWrapper, Tokenizer};

pub struct CharTokenizer<'a> {
    symbols: Vec<&'a str>,
}

impl CharTokenizer<'_> {
    pub fn new() -> Self {
        Self {
            symbols: Token::symbols(),
        }
    }
}

impl Tokenizer for CharTokenizer<'_> {
    fn tokenize(&self, contents: &str, position: &Position) -> Option<TokenWrapper> {
        let view = &contents[position.offset..];
        for &symbol in self.symbols.iter() {
            if !view.starts_with(symbol) {
                continue;
            }

            return Some(TokenWrapper {
                token: Token::from_symbol(symbol),
                span: TokenSpan::new(position.line, position.offset, symbol.len()),
            });
        }

        None
    }
}
