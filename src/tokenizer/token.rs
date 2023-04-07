use crate::parser::Expression;

#[derive(Debug)]
pub struct TokenWrapper {
    pub token: Token,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    If,
    Fun,
    End,
    While,
    True,
    False,
    Return,
}

impl Keyword {
    pub fn all<'a>() -> Vec<&'a str> {
        vec![
            "let", "if", "fun", "end", "while", "true", "false", "return",
        ]
    }
    pub fn from_string(string: &str) -> Keyword {
        match string {
            "let" => Keyword::Let,
            "if" => Keyword::If,
            "fun" => Keyword::Fun,
            "end" => Keyword::End,
            "while" => Keyword::While,
            "true" => Keyword::True,
            "false" => Keyword::False,
            "return" => Keyword::Return,
            _ => todo!("unable to recognize keyword {:?}", string),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    OpenBracket,
    CloseBracket,
    Plus,
    Minus,
    Star,
    Slash,
    OpenParens,
    CloseParens,
    Equals,
    Comma,
    Comment(String),
    Eq,
    Lte,
    Gte,
    Gt,
    Lt,
    Dot,
}

impl Token {
    pub fn symbols<'a>() -> Vec<&'a str> {
        vec![
            "==", ">=", "<=", "<", ">", "=", "+", "-", "*", "/", "(", ")", ",",
            "[", "]",
        ]
    }

    pub fn from_symbol(symbol: &str) -> Token {
        match symbol {
            "==" => Token::Eq,
            ">=" => Token::Gte,
            "<=" => Token::Lte,
            "<" => Token::Lt,
            ">" => Token::Gt,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Star,
            "/" => Token::Slash,
            "(" => Token::OpenParens,
            ")" => Token::CloseParens,
            "[" => Token::OpenBracket,
            "]" => Token::CloseBracket,
            "=" => Token::Equals,
            "," => Token::Comma,
            "." => Token::Dot,
            _ => unreachable!(),
        }
    }

    pub fn value(self) -> Expression {
        match self {
            Token::NumberLiteral(v) => Expression::Number(v),
            Token::BooleanLiteral(v) => Expression::Bool(v),
            Token::StringLiteral(v) => Expression::String(v),
            _ => panic!(""),
        }
    }
}
