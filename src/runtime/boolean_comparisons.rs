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
            | Expression::LessEquals { left, right }
            | Expression::Or { left, right }
            | Expression::And { left, right } => {
                let left = self.execute(*left);
                let right = self.execute(*right);

                (left, right)
            }
            _ => unreachable!(),
        };

        let boolean_value = match (left, right, expr) {
            (Value::Number(a), Value::Number(b), Expression::LessEquals { .. }) => a <= b,
            (Value::Number(a), Value::Number(b), Expression::LessThan { .. }) => a < b,

            (Value::Number(a), Value::Number(b), Expression::GreaterEquals { .. }) => a >= b,
            (Value::Number(a), Value::Number(b), Expression::GreaterThan { .. }) => a > b,


            (a, b, Expression::Or { .. }) => a.is_truthy() || b.is_truthy(),
            (a, b, Expression::And { .. }) => a.is_truthy() && b.is_truthy(),

            (_, _, expr) => unreachable!("invalid boolean comparison with expression {:?}", expr),
        };

        return Value::Bool(boolean_value);
    }
}
