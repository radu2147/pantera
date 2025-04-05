use std::collections::HashMap;
use crate::bytecode::Bytecode;

#[derive(Clone)]
pub struct Env {
    pub enclosing: Option<Box<Env>>,
    pub variables: HashMap<String, Bytecode>
}

impl Env {
    pub fn new_local(env: Box<Env>) -> Self {
        Self {
            enclosing: Some(env),
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
        if self.enclosing.is_none() {
            return self.variables.get(key);
        }
        let var = self.variables.get(key);
        if var.is_none() {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.get_variable(key);
            } else {
                return None;
            }
        }
        var
    }

    pub fn compute_var_key(&self) -> usize {
        if let Some(enc) = &self.enclosing {
            return self.variables.len() + enc.compute_var_key();
        }

        self.variables.len()
    }

    pub fn set_variable(&mut self, key: String) {
        self.variables.insert(key, self.compute_var_key() as Bytecode);
    }
}