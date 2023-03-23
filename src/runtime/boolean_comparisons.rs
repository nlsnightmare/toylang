use crate::parser::Expression;

use super::{value::Value, Runtime};

pub trait BooleanComparisons {
    fn execute_boolean_comparison(&mut self, expr: Expression) -> Value;
}

impl BooleanComparisons for Runtime {
    fn execute_boolean_comparison(&mut self, expr: Expression) -> Value {
        let (left, right) = match expr.clone() {
            Expression::GreaterThan { left, right }
            | Expression::GreaterEquals { left, right }
            | Expression::LessThan { left, right }
            | Expression::LessEquals { left, right } => {
                let left = self.execute(*left);
                let right = self.execute(*right);
                (left, right)
            }
            _ => unreachable!(),
        };

        match (left, right, &expr) {
            (Value::Number(a), Value::Number(b), Expression::LessEquals { .. }) => {
                Value::Bool(a <= b)
            }
            (Value::Number(a), Value::Number(b), Expression::LessThan { .. }) => Value::Bool(a < b),

            (Value::Number(a), Value::Number(b), Expression::GreaterEquals { .. }) => {
                Value::Bool(a >= b)
            }
            (Value::Number(a), Value::Number(b), Expression::GreaterThan { .. }) => {
                Value::Bool(a > b)
            }

            (a, b, _) => panic!("TypeError: unable to compare {:?} and {:?}", &a, &b),
        }
    }
}
