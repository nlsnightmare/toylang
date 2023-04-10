mod bool_expression;
mod expression;

use crate::lexer::token::*;
use bool_expression::BoolExpressionParser;
pub use expression::*;

pub struct Parser {
    tokens: Vec<TokenWrapper>,
    current: usize,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(Token),
    EndOfInput,
}

pub type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<TokenWrapper>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn current_token(&self) -> ParseResult<Token> {
        self.tokens
            .get(self.current)
            .map(|t| t.token.clone())
            .ok_or(ParseError::EndOfInput)
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current + 1).map(|t| t.token.clone())
    }

    fn parse_if_statement(&mut self) -> ParseResult<Expression> {
        self.try_consume(Token::Keyword(Keyword::If))?;

        let condition = Box::new(self.parse_expression()?);

        let mut body: AST = Vec::new();
        loop {
            if let Ok(Token::Keyword(Keyword::End)) = self.current_token() {
                self.try_consume(Token::Keyword(Keyword::End))?;
                break;
            }

            let expr = Box::new(self.parse_expression()?);
            body.push(expr);
        }

        Ok(Expression::IfCondition { condition, body })
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        let token = self.current_token()?;

        let parsed = match token {
            Token::Exclamation => {
                self.consume();
                let expr = self.parse_expression()?;

                // This really needs to factor in operator precedence
                Ok(Expression::BoolNegation(Box::new(expr)))
            }
            Token::Keyword(Keyword::If) => self.parse_if_statement(),
            Token::Keyword(Keyword::Return) => self.parse_return_statement(),
            Token::Keyword(Keyword::Let) => self.parse_variable_decleration(),
            Token::Keyword(Keyword::Fun) => self.parse_function_decleration(),
            Token::Keyword(Keyword::While) => self.parse_while_loop(),
            Token::BooleanLiteral(_) | Token::StringLiteral(_) | Token::NumberLiteral(_) => {
                self.consume();
                Ok(token.value())
            }
            Token::OpenBracket => self.parse_array(),
            Token::Identifier(name) => match self.peek() {
                // TODO: implement fields
                Some(Token::Equals) => self.parse_variable_assignment(),
                Some(Token::OpenParens) => self.parse_function_call(),
                Some(Token::OpenBracket) => {
                    self.consume(); // The identifier
                    self.try_consume(Token::OpenBracket)?;
                    let index = self.parse_expression()?;
                    self.try_consume(Token::CloseBracket)?;

                    Ok(Expression::ArrayIndexing {
                        array: Box::new(Expression::Variable(name.to_owned())),
                        index: Box::new(index),
                    })
                }
                _ => {
                    self.consume();

                    Ok(Expression::Variable(name.to_owned()))
                }
            },
            _ => Err(ParseError::InvalidToken(token)),
        }?;

        match self.current_token() {
            Ok(Token::Plus) => self.parse_addition(parsed),
            Ok(Token::Minus) => self.parse_subtraction(parsed),
            Ok(Token::Star) => self.parse_multiplication(parsed),
            Ok(Token::Slash) => self.parse_division(parsed),
            Ok(Token::Lte | Token::Lt | Token::Gte | Token::Gt | Token::And | Token::Or) => {
                self.parse_boolean_expression(parsed)
            }
            _ => Ok(parsed),
        }
    }

    pub fn parse(&mut self) -> ParseResult<AST> {
        // I don't like that i have to do this here, but hey
        self.tokens.retain(|t| match t.token {
            Token::Comment(_) => false,
            _ => true,
        });

        let mut ast: AST = Vec::new();
        while let Ok(_) = self.current_token() {
            let expression = self.parse_expression()?;
            ast.push(Box::new(expression));
        }

        Ok(ast)
    }

    fn identifier_name(&mut self, token: Token) -> ParseResult<String> {
        match token {
            Token::Identifier(ref n) => {
                self.consume();
                Ok(n.to_owned())
            }
            _ => Err(ParseError::InvalidToken(token)),
        }
    }

    fn try_consume(&mut self, token: Token) -> ParseResult<()> {
        let current = self.current_token()?;

        if current == token {
            self.current += 1;
            return Ok(());
        }

        Err(ParseError::InvalidToken(current))
    }

    fn consume(&mut self) {
        self.current += 1;
    }

    fn parse_while_loop(&mut self) -> ParseResult<Expression> {
        self.try_consume(Token::Keyword(Keyword::While))?;

        let condition = self.parse_expression()?;

        let mut body = Vec::new();
        // TODO: abstract this
        loop {
            if let Ok(Token::Keyword(Keyword::End)) = self.current_token() {
                self.try_consume(Token::Keyword(Keyword::End))?;
                break;
            }

            let expr = Box::new(self.parse_expression()?);
            body.push(expr);
        }

        Ok(Expression::WhileLoop {
            condition: Box::new(condition),
            body,
        })
    }

    fn parse_function_decleration(&mut self) -> ParseResult<Expression> {
        self.try_consume(Token::Keyword(Keyword::Fun))?;

        let name_token = self.current_token()?;

        let name = match name_token {
            Token::Identifier(_) => self.identifier_name(name_token),
            Token::OpenParens => Ok("".to_owned()), // anonymous function
            _ => Err(ParseError::InvalidToken(name_token.clone())),
        }?;

        let mut arguments = Vec::new();
        self.try_consume(Token::OpenParens)?;
        while let Ok(Token::Identifier(name)) = self.current_token() {
            arguments.push(name);
            self.consume();

            if let Some(Token::Identifier(_)) = self.peek() {
                self.try_consume(Token::Comma)?;
            }
        }
        self.try_consume(Token::CloseParens)?;

        let mut body = Vec::new();

        loop {
            if let Ok(Token::Keyword(Keyword::End)) = self.current_token() {
                break;
            }

            let expr = Box::new(self.parse_expression()?);
            body.push(expr);
        }

        self.try_consume(Token::Keyword(Keyword::End))?;

        Ok(Expression::FunctionDefinition {
            name,
            body,
            arguments,
        })
    }

    fn parse_function_call(&mut self) -> ParseResult<Expression> {
        let current = self.current_token()?;

        let name = self.identifier_name(current)?;

        self.try_consume(Token::OpenParens)?;

        let mut arguments = Vec::new();

        // no arguments
        if let Ok(_) = self.try_consume(Token::CloseParens) {
            Ok(Expression::FunctionCall { name, arguments })
        } else {
            let argument = self.parse_expression()?;
            arguments.push(Box::new(argument));

            // TODO: make a parse_arguments function
            while self.try_consume(Token::Comma).is_ok() {
                let argument = self.parse_expression()?;
                arguments.push(Box::new(argument));
            }

            self.try_consume(Token::CloseParens)?;

            Ok(Expression::FunctionCall { name, arguments })
        }
    }

    fn parse_addition(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Plus)?;
        let right = self.parse_expression()?;

        Ok(Expression::Addition {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_subtraction(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Minus)?;
        let right = self.parse_expression()?;

        Ok(Expression::Subtraction {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_multiplication(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Star)?;
        let right = self.parse_expression()?;

        Ok(Expression::Multiplication {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_division(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Slash)?;
        let right = self.parse_expression()?;

        Ok(Expression::Division {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_array(&mut self) -> ParseResult<Expression> {
        self.try_consume(Token::OpenBracket)?;

        let mut items = vec![];

        loop {
            match self.current_token() {
                Ok(Token::CloseBracket) => break,
                Ok(Token::Comma) => self.try_consume(Token::Comma)?, // no trailing commas for now
                _ => {}
            };

            let expr = self.parse_expression()?;

            items.push(Box::new(expr));
        }

        self.try_consume(Token::CloseBracket)?;

        Ok(Expression::Array(items))
    }

    fn parse_variable_assignment(&mut self) -> ParseResult<Expression> {
        let name = self.identifier_name(self.current_token()?)?;

        self.try_consume(Token::Equals)?;
        let value = Box::new(self.parse_expression()?);

        Ok(Expression::VariableAssignment { name, value })
    }

    fn parse_return_statement(&mut self) -> ParseResult<Expression> {
        self.try_consume(Token::Keyword(Keyword::Return))?;

        // TODO: check case of empty return;
        let value = self.parse_expression()?;

        Ok(Expression::Return(Box::new(value)))
    }

    fn parse_variable_decleration(&mut self) -> ParseResult<Expression> {
        self.try_consume(Token::Keyword(Keyword::Let))?;

        let current = self.current_token()?;

        let name = self.identifier_name(current)?.clone();

        self.try_consume(Token::Equals)?;

        let value = Box::new(self.parse_expression()?);

        Ok(Expression::VariableDecleration { name, value })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::{Keyword, Token, TokenSpan, TokenWrapper},
        parser::Expression,
    };

    use super::Parser;

    fn wrap_tokens(tokens: Vec<Token>) -> Vec<TokenWrapper> {
        tokens
            .into_iter()
            .map(|token| TokenWrapper {
                token,
                span: TokenSpan {
                    line: 0,
                    start: 0,
                    len: 10,
                },
            })
            .collect()
    }

    #[test]
    fn string_variable_decleration() {
        let tokens = wrap_tokens(vec![
            Token::Keyword(Keyword::Let),
            Token::Identifier("variable".to_owned()),
            Token::Equals,
            Token::StringLiteral(" ".to_owned()),
        ]);

        let expression = Parser::new(tokens).parse().unwrap().pop().unwrap();

        assert_eq!(
            *expression,
            Expression::VariableDecleration {
                name: "variable".to_owned(),
                value: Box::new(Expression::String(" ".to_owned()))
            }
        );
    }

    #[test]
    fn string_variable_assignment() {
        let tokens = wrap_tokens(vec![
            Token::Identifier("variable".to_owned()),
            Token::Equals,
            Token::StringLiteral(" ".to_owned()),
        ]);

        let expression = Parser::new(tokens).parse().unwrap().pop().unwrap();

        assert_eq!(
            *expression,
            Expression::VariableAssignment {
                name: "variable".to_owned(),
                value: Box::new(Expression::String(" ".to_owned()))
            },
        );
    }

    #[test]
    fn function_definition() {
        let tokens = wrap_tokens(vec![
            Token::Keyword(Keyword::Fun),
            Token::Identifier("some_function".to_owned()),
            Token::OpenParens,
            Token::CloseParens,
            Token::Keyword(Keyword::End),
        ]);

        let expression = Parser::new(tokens).parse().unwrap().pop().unwrap();

        assert_eq!(
            *expression,
            Expression::FunctionDefinition {
                name: "some_function".to_owned(),
                body: Vec::new(),
                arguments: Vec::new(),
            }
        )
    }
}
