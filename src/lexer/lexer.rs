use regex::Regex;

use super::{
    tokenizers::{
        CharTokenizer, CommentTokenizer, IdentifierTokenizer, KeywordTokenizer, NumberTokenizer,
        StringTokenizer, WhitespaceTokenizer, Tokenizer,
    },
    TokenWrapper, Token,
};

pub struct Lexer;

pub struct Position {
    pub line: usize,
    pub offset: usize,
}

impl Position {
    pub fn advance(&mut self, amount: usize) {
        self.offset += amount;
    }
}

impl Lexer {

    pub fn tokenize(source_code: String) -> Vec<TokenWrapper> {
        let cloned = source_code.clone();
        let lines = cloned.split('\n');

        let tokenizers: Vec<Box<dyn Tokenizer>> = vec![
            Box::new(CommentTokenizer::new()),
            Box::new(KeywordTokenizer::new()),
            Box::new(CharTokenizer::new()),
            Box::new(StringTokenizer::new()),
            Box::new(IdentifierTokenizer::new()),
            Box::new(NumberTokenizer::new()),
            Box::new(WhitespaceTokenizer::new()),
        ];

        let mut tokens = vec![];
        for (line, contents) in lines.enumerate() {
            let mut position = Position { line, offset: 0 };
            'line: loop {
                let view: &str = &contents[position.offset..contents.len()];
                if view.len() == 0 {
                    break;
                }

                for t in &tokenizers {
                    if let Some(wrapper) = t.tokenize(contents, &position) {
                        position.advance(wrapper.len());
                        tokens.push(wrapper);
                        continue 'line;
                    }
                }

                panic!(
                    "Unable to generate a token from {:?} [line: {}]",
                    view, position.offset
                );
            }
        }

        tokens
            .into_iter()
            .filter(|t| t.token != Token::Whitespace)
            .collect()
    }
}
