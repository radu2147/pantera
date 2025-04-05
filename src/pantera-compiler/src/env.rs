use std::collections::HashMap;
use crate::bytecode::Bytecode;

pub struct Env {
    pub enclosing: Option<Box<Env>>,
    pub variables: HashMap<String, Bytecode>
}

impl Env {
    pub fn new_local(env: Env) -> Self {
        Self {
            enclosing: Some(Box::new(env)),
            variables: HashMap::new()
        }
    }

    pub fn new() -> Self {
        Self {
            enclosing: None,
            variables: HashMap::new()
        }
    }

    pub fn get_variable(&self, key: &str) -> Option<&Bytecode> {
        self.variables.get(key)
    }

    pub fn set_variable(&mut self, key: String) {
        self.variables.insert(key, self.variables.len() as Bytecode);
    }
}