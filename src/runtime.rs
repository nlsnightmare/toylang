mod boolean_comparisons;
mod builtin;
mod math_operations;
mod value;

use std::collections::HashMap;

use crate::parser::{Expression, AST};

use self::{
    boolean_comparisons::BooleanComparisons,
    builtin::{execute_builtin, is_builtin},
    math_operations::MathOperations,
    value::Value,
};

#[derive(Debug)]
pub struct Runtime {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Value>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            variables: HashMap::new(),
            functions: HashMap::new(),
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

    fn execute(&mut self, expr: Expression) -> Value {
        match expr {
            Expression::Variable(name) => {
                if let Some(value) = self.variables.get(&name) {
                    value.clone()
                } else {
                    panic!("undefined variable name {:?}", name);
                }
            }

            Expression::VariableDecleration { name, value } => {
                let value = self.execute(*value);
                self.variables.insert(name, value.clone());

                value
            }

            Expression::VariableAssignment { name, value } => {
                match self.variables.get(&name) {
                    Some(_) => {
                        let value = self.execute(*value);
                        self.variables.insert(name, value);

                        Value::Void
                    }
                    None => panic!("unknown variable {}", name),
                };
                Value::Void
            }

            Expression::Addition { .. }
            | Expression::Subtraction { .. }
            | Expression::Multiplication { .. }
            | Expression::Division { .. } => self.execute_math_operation(expr),

            Expression::GreaterThan { .. }
            | Expression::GreaterEquals { .. }
            | Expression::LessThan { .. }
            | Expression::LessEquals { .. } => self.execute_boolean_comparison(expr),

            Expression::String(v) => Value::String(v),
            Expression::Bool(v) => Value::Bool(v),
            Expression::Number(v) => Value::Number(v),

            Expression::IfCondition { condition, body } => {
                if self.execute(*condition).is_truthy() {
                    self.run(body.clone());
                }

                Value::Void
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
                let value = Value::Function { body, arguments };

                if name != "" {
                    self.functions.insert(name, value.clone());
                }

                value
            }
            Expression::FunctionCall { name, arguments } => {
                let argument_values = arguments
                    .into_iter()
                    .map(|a| self.execute(*a))
                    .collect::<Vec<Value>>();

                if is_builtin(&name) {
                    execute_builtin(&name, argument_values)
                } else {
                    let (body, arguments) =
                        match (self.functions.get(&name), self.variables.get(&name)) {
                            (Some(Value::Function { body, arguments }), _) => (body, arguments),
                            (None, Some(Value::Function { body, arguments })) => (body, arguments),
                            _ => panic!("undefined function {}", name),
                        };

                    // FIXME: there must be a better way to create  a new scope than this lmao
                    let mut runtime = Runtime::new();
                    runtime.functions = self.functions.clone();
                    for (argument, value) in arguments.iter().zip(argument_values) {
                        runtime.variables.insert(argument.to_owned(), value);
                    }

                    runtime.run(body.clone())
                }
            }
            _ => todo!("no idea what to do with {:?}", expr),
        }
    }
}
