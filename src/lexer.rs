pub mod token;

mod lexer;
mod tokenizers;

pub use lexer::Lexer;
pub use token::*;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::lexer::Lexer;

    use super::{Keyword, Token};

    /// Helper function which makes sure that the tokens of a source code are the expected ones.
    fn verify_tokens(contents: &str, expected_tokens: Vec<Token>) {
        let tokens = Lexer::tokenize(contents.to_owned());

        assert_eq!(tokens.len(), expected_tokens.len());

        tokens
            .iter()
            .map(|t| t.token.clone())
            .zip(expected_tokens)
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }

    #[test]
    fn string_literal_test() {
        verify_tokens(
            "\"hello world\"",
            vec![Token::StringLiteral("hello world".to_owned())],
        );
    }

    #[test]
    fn identifier_test() {
        verify_tokens(
            "let name",
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier("name".to_owned()),
            ],
        );
    }

    #[test]
    fn comment_test() {
        verify_tokens(
            "# hello from the comment",
            vec![Token::Comment("# hello from the comment".to_owned())],
        );
    }

    #[test]
    fn number_variable_assignment() {
        verify_tokens(
            "let age = 2",
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
        verify_tokens(
            "let name = \"nick\"",
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
        verify_tokens(
            "print(\"Hello world\")",
            vec![
                Token::Identifier("print".to_owned()),
                Token::OpenParens,
                Token::StringLiteral("Hello world".to_owned()),
                Token::CloseParens,
            ],
        );
    }

    #[test]
    fn array_decleration() {
        verify_tokens(
            "[1,2,3]",
            vec![
                Token::OpenBracket,
                Token::NumberLiteral(1.0),
                Token::Comma,
                Token::NumberLiteral(2.0),
                Token::Comma,
                Token::NumberLiteral(3.0),
                Token::CloseBracket,
            ],
        );
    }

    #[test]
    fn addition() {
        verify_tokens(
            "1 + a",
            vec![
                Token::NumberLiteral(1.0),
                Token::Plus,
                Token::Identifier("a".to_owned()),
            ],
        );
    }
}
