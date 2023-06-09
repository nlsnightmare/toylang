use crate::parser::AST;

#[derive(Debug, Clone)]
pub enum Value {
    Array(ArrayValue),
    Bool(bool),
    Function(FunctionValue),
    Number(f64),
    String(String),
    Void,
}

#[derive(Clone, Debug)]
pub struct ArrayValue {
    pub contents: Vec<Box<Value>>,
    pub length: usize,
}

#[derive(Clone, Debug)]
pub struct FunctionValue {
    pub arguments: Vec<String>,
    pub body: AST,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match *self {
            Value::Number(i) => i != 0.0,
            Value::String(ref s) => s.is_empty(),
            Value::Bool(v) => v,
            _ => panic!("{:?} is not a valid condition", &self),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Value::Bool(true) => "true".to_owned(),
            Value::Bool(false) => "false".to_owned(),
            Value::Number(i) => i.to_string(),
            Value::String(ref v) => v.clone(),
            _ => panic!("Value {:?} cannot be converted to string", &self),
        }
    }
}
