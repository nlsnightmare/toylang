mod boolean_comparisons;
mod builtin;
mod math_operations;
mod scope;
mod value;

use std::collections::VecDeque;

use crate::parser::{Expression, AST};

use self::{
    boolean_comparisons::BooleanComparisons,
    builtin::{execute_builtin, is_builtin},
    math_operations::MathOperations,
    scope::Scope,
    value::{ArrayValue, FunctionValue, Value},
};

#[derive(Debug)]
pub struct Runtime {
    global_scope: Scope,
    local_scope: VecDeque<Scope>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            global_scope: Scope::default(),
            local_scope: VecDeque::default(),
        }
    }

    pub fn run(&mut self, ast: AST) -> Value {
        let mut last_value = Value::Void;

        for expr in ast {
            if let Expression::Return(inner) = *expr {
                return self.execute(*inner);
            }

            last_value = self.execute(*expr);
        }

        last_value
    }

    fn try_get_variable(&self, name: &str) -> Option<Value> {
        self.global_scope
            .get(name)
            .or_else(|| self.local_scope.get(0)?.get(name))
            .map(|v| v.clone())
    }

    fn mutate_variable<CB>(&mut self, name: &str, mutation_fn: CB)
    where
        CB: FnOnce(Value) -> Value,
    {
        let value = self.get_variable(name);
        let result = mutation_fn(value);
        self.set_variable(name, result);
    }

    fn get_variable(&self, name: &str) -> Value {
        self.try_get_variable(name)
            .expect(&format!("Undefined variable '{}'!", name))
    }

    fn set_variable(&mut self, name: &str, value: Value) {
        let value = value.clone();
        let name = name.to_owned();

        if let Some(mut s) = self.local_scope.pop_front() {
            s.set(name, value);

            self.local_scope.push_front(s);
            return;
        }

        self.global_scope.set(name, value);
    }

    fn execute(&mut self, expr: Expression) -> Value {
        match expr {
            Expression::Variable(name) => self.get_variable(&name),

            Expression::VariableDecleration { name, value } => {
                let value = self.execute(*value);

                self.set_variable(&name, value.clone());

                value
            }

            Expression::VariableAssignment { name, value } => {
                let value = self.execute(*value);

                if self.try_get_variable(&name).is_none() {
                    panic!("Undefined variable {}", name);
                }

                self.set_variable(&name, value.clone());

                value
            }

            Expression::Addition { .. }
            | Expression::Subtraction { .. }
            | Expression::Multiplication { .. }
            | Expression::Division { .. } => self.execute_math_operation(expr),

            Expression::BoolNegation(v) => {
                let v = self.execute(*v).is_truthy();

                Value::Bool(!v)
            }

            Expression::GreaterThan { .. }
            | Expression::GreaterEquals { .. }
            | Expression::LessThan { .. }
            | Expression::LessEquals { .. }
            | Expression::And { .. }
            | Expression::Or { .. } => self.execute_boolean_comparison(expr),

            Expression::String(v) => Value::String(v),
            Expression::Bool(v) => Value::Bool(v),
            Expression::Number(v) => Value::Number(v),
            Expression::ArrayAssignment {
                identifier,
                index,
                value,
            } => {
                let value = self.execute(*value);
                let index = self.execute(*index);

                self.mutate_variable(&identifier, |v| match (v, index) {
                    (Value::Array(mut arr), Value::Number(i)) => {
                        arr.contents[i as usize] = Box::new(value.clone());
                        Value::Array(arr)
                    }
                    _ => panic!("{:?} is not an array!", identifier),
                });

                value
            }
            Expression::ArrayIndexing {
                identifier: array,
                index,
            } => {
                let value = self.execute(*array);
                let index = self.execute(*index);

                match [&value, &index] {
                    [Value::Array(arr), Value::Number(i)] => *arr.contents[*i as usize].clone(),
                    _ => panic!(
                        "something went wrong with indexing lol, {:#?}, {:?}",
                        value, index
                    ),
                }
            }
            Expression::Array(exprs) => {
                let values = exprs
                    .into_iter()
                    .map(|expr| self.execute(*expr))
                    .map(|v| Box::new(v))
                    .collect::<Vec<_>>();

                Value::Array(ArrayValue {
                    length: values.len(),
                    contents: values,
                })
            }

            Expression::IfCondition { condition, body } => {
                if self.execute(*condition).is_truthy() {
                    self.run(body.clone())
                } else {
                    Value::Void
                }
            }

            Expression::WhileLoop { condition, body } => {
                while self.execute(*condition.clone()).is_truthy() {
                    self.run(body.clone());
                }

                Value::Void
            }
            Expression::FunctionDefinition {
                name,
                body,
                arguments,
            } => {
                let value = Value::Function(FunctionValue { body, arguments });

                if !name.is_empty() {
                    self.set_variable(&name, value.clone());
                }

                value
            }
            Expression::FunctionCall { name, arguments } => {
                let argument_values = arguments
                    .into_iter()
                    .map(|a| self.execute(*a))
                    .collect::<Vec<Value>>();

                if is_builtin(&name) {
                    return execute_builtin(&name, argument_values);
                }

                if let Value::Function(FunctionValue { body, arguments }) = self.get_variable(&name)
                {
                    let mut scope = Scope::default();
                    for (arg, value) in arguments.iter().zip(argument_values) {
                        scope.set(arg.clone(), value);
                    }

                    self.local_scope.push_front(scope);
                    let value = self.run(body.clone());
                    self.local_scope.pop_front();

                    value
                } else {
                    panic!("undefined function name {:?}", name);
                }
            }
            Expression::Return(_) => unreachable!(),
        }
    }
}
