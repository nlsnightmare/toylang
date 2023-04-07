use regex::Regex;
pub mod token;

pub use token::*;

pub struct Tokenizer;

impl Tokenizer {
    fn consume(view: &str, regex: &Regex) -> Option<(String, usize)> {
        let groups = regex.captures(&view)?;

        let first = groups.get(0)?;
        let captured = groups.get(1)?;

        let full_captured = first.end() - first.start();

        let text = view[captured.start()..captured.end()].to_owned();

        Some((text, full_captured))
    }

    pub fn tokenize(source_code: String) -> Vec<TokenWrapper> {
        let cloned = source_code.clone();
        let lines = cloned.split('\n');

        let keyword_regex =
            Regex::new(&format!(r"^({})", Keyword::all().join("|")).to_owned()).unwrap();
        let identifier_regex = Regex::new(r"^([a-zA-Z][a-zA-Z0-9_$]*)").unwrap();
        let symbols = Token::symbols();
        let string_regex = Regex::new(r"^'(.*)'").unwrap();
        let number_regex = Regex::new(r"^([0-9]+\.?[0-9]*)").unwrap();
        let whitespace_regex = Regex::new(r"^(\s+)").unwrap();
        let comment_regex = Regex::new(r"^(#.*$)").unwrap();

        let mut tokens = vec![];
        for (line, contents) in lines.enumerate() {
            let mut position: usize = 0;
            'line: loop {
                let view: &str = &contents[position..contents.len()];
                if view.len() == 0 {
                    break;
                }

                for &symbol in symbols.iter() {
                    if view.starts_with(symbol) {
                        position = position + 1;
                        tokens.push(TokenWrapper {
                            token: Token::from_symbol(symbol),
                            line,
                        });
                        continue 'line;
                    }
                }

                if let Some((text, size)) = Tokenizer::consume(view, &keyword_regex) {
                    let keyword = Keyword::from_string(&text);

                    let token = match keyword {
                        Keyword::True => Token::BooleanLiteral(true),
                        Keyword::False => Token::BooleanLiteral(false),
                        _ => Token::Keyword(keyword),
                    };

                    tokens.push(TokenWrapper { token, line });

                    position = position + size;
                } else if let Some((text, size)) = Tokenizer::consume(view, &number_regex) {
                    tokens.push(TokenWrapper {
                        token: Token::NumberLiteral(text.parse::<f64>().unwrap()),
                        line,
                    });
                    position = position + size;
                } else if let Some((text, size)) = Tokenizer::consume(view, &comment_regex) {
                    tokens.push(TokenWrapper {
                        token: Token::Comment(text),
                        line,
                    });
                    position = position + size;
                } else if let Some((text, size)) = Tokenizer::consume(view, &identifier_regex) {
                    tokens.push(TokenWrapper {
                        token: Token::Identifier(text),
                        line,
                    });

                    position = position + size;
                } else if let Some((text, size)) = Tokenizer::consume(view, &string_regex) {
                    tokens.push(TokenWrapper {
                        token: Token::StringLiteral(text),
                        line,
                    });

                    position = position + size;
                } else if let Some((_, size)) = Tokenizer::consume(view, &whitespace_regex) {
                    position = position + size;
                } else {
                    panic!("Unable to parse {:?}", view);
                }

                // skip the newline
                if position >= contents.len() {
                    break;
                }
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::*;
    use super::Token;

    fn verify_tokens(contents: String, expected_tokens: Vec<Token>) {
        let tokens = Tokenizer::tokenize(contents.to_owned());
        dbg!(&tokens);

        assert_eq!(tokens.len(), expected_tokens.len());

        tokens
            .iter()
            .map(|t| t.token.clone())
            .zip(expected_tokens)
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }

    #[test]
    fn string_literal_test() {
        let contents = "'hello world'";

        verify_tokens(
            contents.to_owned(),
            vec![Token::StringLiteral("hello world".to_owned())],
        );
    }

    #[test]
    fn identifier_test() {
        let contents = "let name";

        verify_tokens(
            contents.to_owned(),
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier("name".to_owned()),
            ],
        );
    }

    #[test]
    fn comment_test() {
        let contents = "# hello from the comment";

        verify_tokens(
            contents.to_owned(),
            vec![Token::Comment("# hello from the comment".to_owned())],
        );
    }

    #[test]
    fn number_variable_assignment() {
        let contents = "let age = 2";

        verify_tokens(
            contents.to_owned(),
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier("age".to_owned()),
                Token::Equals,
                Token::NumberLiteral(2.0),
            ],
        );
    }

    #[test]
    fn string_variable_assignment() {
        let contents = "let name = 'nick'";

        verify_tokens(
            contents.to_owned(),
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier("name".to_owned()),
                Token::Equals,
                Token::StringLiteral("nick".to_owned()),
            ],
        );
    }

    #[test]
    fn function_call() {
        let contents = "print('Hello world')".to_owned();

        verify_tokens(
            contents,
            vec![
                Token::Identifier("print".to_owned()),
                Token::OpenParens,
                Token::StringLiteral("Hello world".to_owned()),
                Token::CloseParens,
            ],
        );
    }
}
