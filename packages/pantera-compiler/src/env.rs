use std::collections::HashMap;
use crate::bytecode::Bytecode;

#[derive(Debug, Clone)]
pub struct Variable {
    pub key: Bytecode,
    pub is_constant: bool
}

#[derive(Debug, Clone)]
pub struct Env {
    pub enclosing: Option<Box<Env>>,
    pub variables: HashMap<String, Variable>,
    pub frame_beginning: bool
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new_local(env: Box<Env>) -> Self {
        Self {
            enclosing: Some(env),
            variables: HashMap::new(),
            frame_beginning: false
        }
    }

    pub fn new_frame(env: Box<Env>) -> Self {
        Self {
            enclosing: Some(env),
            variables: HashMap::new(),
            frame_beginning: true
        }
    }

    pub fn new() -> Self {
        Self {
            enclosing: None,
            variables: HashMap::new(),
            frame_beginning: true
        }
    }

    pub fn get_variable(&self, key: &str) -> Option<&Variable> {
        if self.frame_beginning || self.enclosing.is_none() {
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

    fn compute_var_key(&self) -> usize {
        if self.frame_beginning {
            return self.variables.len();
        }
        if let Some(enc) = &self.enclosing {
            return self.variables.len() + enc.compute_var_key();
        }

        self.variables.len()
    }

    fn set_variable_internal(&mut self, key: String, is_constant: bool) {
        self.variables.insert(key, Variable{key: self.compute_var_key() as Bytecode, is_constant});
    }

    pub fn set_variable(&mut self, key: String) {
        self.set_variable_internal(key, false)
    }

    pub fn set_constant(&mut self, key: String) {
        self.set_variable_internal(key, true);
    }
}