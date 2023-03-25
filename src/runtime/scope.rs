use super::value::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Scope {
    variables: HashMap<String, Value>,
}

impl Scope {
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}
