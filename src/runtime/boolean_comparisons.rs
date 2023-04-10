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
                let left = self.execute(*left).is_truthy();
                let right = self.execute(*right).is_truthy();

                (left, right)
            }
            _ => unreachable!(),
        };

        let boolean_value = match expr {
            Expression::LessEquals { .. } => left <= right,
            Expression::LessThan { .. } => left < right,

            Expression::GreaterEquals { .. } => left <= right,
            Expression::GreaterThan { .. } => left < right,

            Expression::Or { .. } => left || right,
            Expression::And { .. } => left && right,

            _ => unreachable!("invalid boolean comparison with expression {:?}", expr),
        };

        return Value::Bool(boolean_value);
    }
}
