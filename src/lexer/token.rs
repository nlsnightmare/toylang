use crate::parser::Expression;

#[derive(Debug, Default)]
pub struct TokenSpan {
    pub line: usize,
    pub start: usize,
    pub len: usize,
}

impl TokenSpan {
    pub fn new(line: usize, start: usize, len: usize) -> Self {
        TokenSpan { line, start, len }
    }
}

#[derive(Debug)]
pub struct TokenWrapper {
    pub token: Token,
    pub span: TokenSpan,
}

impl TokenWrapper {
    pub fn len(&self) -> usize {
        self.span.len
    }
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
    Whitespace,
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
    And,
    Or,
    Exclamation,
}

impl Token {
    pub fn symbols<'a>() -> Vec<&'a str> {
        vec![
            "==", ">=", "<=", "<", ">", "=", "+", "-", "*", "/", "(", ")", ",", "[", "]",
            "&&", "||", "!"
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
            "&&" => Token::And,
            "||" => Token::Or,
            "!" => Token::Exclamation,
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
