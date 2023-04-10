use crate::lexer::token::Token;

use super::{Expression, ParseResult, Parser};

pub trait BoolExpressionParser {
    fn parse_boolean_expression(&mut self, left: Expression) -> ParseResult<Expression>;
    fn parse_less_than(&mut self, left: Expression) -> ParseResult<Expression>;
    fn parse_less_than_equals(&mut self, left: Expression) -> ParseResult<Expression>;

    fn parse_greater_than(&mut self, left: Expression) -> ParseResult<Expression>;
    fn parse_greater_than_equals(&mut self, left: Expression) -> ParseResult<Expression>;

    fn parse_negation(&mut self, left: Expression) -> ParseResult<Expression>;
    fn parse_and(&mut self, left: Expression) -> ParseResult<Expression>;
    fn parse_or(&mut self, left: Expression) -> ParseResult<Expression>;
}

impl BoolExpressionParser for Parser {
    fn parse_less_than(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Lt)?;
        let right = self.parse_expression()?;

        Ok(Expression::LessThan {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_less_than_equals(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Lte)?;
        let right = self.parse_expression()?;

        Ok(Expression::LessEquals {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_greater_than(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Gt)?;
        let right = self.parse_expression()?;

        Ok(Expression::GreaterThan {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_greater_than_equals(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Gte)?;
        let right = self.parse_expression()?;

        Ok(Expression::GreaterEquals {
            left: Box::new(left),
            right: Box::new(right),
        })
    }


    fn parse_negation(&mut self, left: Expression) -> ParseResult<Expression> {
        todo!("implement negation");
    }

    fn parse_or(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::Or)?;
        let right = self.parse_expression()?;

        Ok(Expression::Or {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_and(&mut self, left: Expression) -> ParseResult<Expression> {
        self.try_consume(Token::And)?;
        let right = self.parse_expression()?;

        Ok(Expression::And {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    fn parse_boolean_expression(&mut self, left: Expression) -> ParseResult<Expression> {
        let next = self.current_token()?;

        match next {
            Token::Lte => self.parse_less_than_equals(left),
            Token::Lt => self.parse_less_than(left),
            Token::Gte => self.parse_greater_than_equals(left),
            Token::Gt => self.parse_greater_than(left),

            Token::And => self.parse_and(left),
            Token::Or => self.parse_or(left),
            _ => unreachable!("invalid token {:?} when parsing bool expression", next),
        }
    }
}
