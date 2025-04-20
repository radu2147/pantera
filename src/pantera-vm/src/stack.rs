use crate::value::Value;

#[derive(Debug)]
pub struct Stack {
    elements: Vec<Value>,
    top: usize
}

impl Stack {
    pub fn init() -> Self {
        Self{
            elements: vec![Value::Null; 50],
            top: 0usize
        }
    }
    pub fn push(&mut self, el: Value) {
        self.elements[self.top] = el;
        self.top += 1;
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index)
    }

    pub fn set(&mut self, index: usize, el: Value) {
        self.elements[index] = el;
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.top == 0 {
            return None;
        }
        let el = self.elements[self.top - 1].clone();
        self.top -= 1;
        Some(el)
    }

    pub fn peek(&self) -> Option<&Value> {
        if self.top == 0 {
            return None;
        }
        Some(&self.elements[self.top])
    }

    pub fn len(&self) -> usize {
        self.top
    }
}