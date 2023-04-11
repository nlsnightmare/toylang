use crate::parser::Expression::{self, Addition, Division, Multiplication, Subtraction};

use super::{value::Value, Runtime};

pub trait MathOperations {
    fn execute_math_operation(&mut self, expr: Expression) -> Value;
}

impl MathOperations for Runtime {
    fn execute_math_operation(&mut self, expr: Expression) -> Value {
        let (left, right) = match expr.clone() {
            Addition(op) | Subtraction(op) | Multiplication(op) | Division(op) => {
                (self.execute(*op.left), self.execute(*op.right))
            }
            _ => unreachable!(),
        };

        match (left, right, &expr) {
            (Value::Number(a), Value::Number(b), Addition { .. }) => Value::Number(a + b),
            (Value::String(a), Value::String(b), Addition { .. }) => {
                Value::String(format!("{}{}", a, b))
            }
            (Value::String(a), b, Addition { .. }) => {
                Value::String(format!("{}{}", a, b.to_string()))
            }
            (a, Value::String(b), Addition { .. }) => {
                Value::String(format!("{}{}", a.to_string(), b))
            }

            (a, b, Addition { .. }) => panic!("TypeError: unable to add {:?} and {:?}", &a, &b),

            (Value::Number(a), Value::Number(b), Subtraction { .. }) => Value::Number(a - b),
            (a, b, Subtraction { .. }) => {
                panic!("TypeError: unable to subtract {:?} and {:?}", &a, &b)
            }

            (Value::Number(a), Value::Number(b), Multiplication { .. }) => Value::Number(a * b),
            (a, b, Multiplication { .. }) => {
                panic!("TypeError: unable to multiply {:?} and {:?}", &a, &b)
            }

            (Value::Number(a), Value::Number(b), Division { .. }) => Value::Number(a / b),
            (a, b, Division { .. }) => panic!("TypeError: unable to divide {:?} and {:?}", &a, &b),
            _ => unreachable!(),
        }
    }
}
