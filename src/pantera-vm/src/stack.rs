use std::io::Write;
use pantera_heap::value::Value;

#[derive(Debug)]
pub struct Stack {
    pub elements: Vec<Value>,
    top: usize,
    pub offset: usize
}

impl Stack {
    pub fn init() -> Self {
        Self{
            elements: vec![Value::Null; 50],
            top: 0usize,
            offset: 0usize
        }
    }
    pub fn push(&mut self, el: Value) {
        if self.top >= self.elements.len() - 20 {
            self.elements.resize(2 * self.elements.len(), Value::Null);
        }
        self.elements[self.top] = el;
        self.top += 1;
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index + self.offset)
    }

    pub fn debug(&self) {
        for i in 0..self.top {
            print!("{:?}, ", self.elements.get(i).unwrap())
        }
        println!("END");
        std::io::stdout().flush().unwrap();
    }

    pub fn set(&mut self, index: i32, el: Value) {
        self.elements[(index + self.offset as i32) as usize] = el;
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.top == self.offset {
            return None;
        }
        let el = self.elements[self.top - 1].clone();
        self.top -= 1;
        Some(el)
    }

    pub fn reset_to(&mut self, index: usize) {
        self.top = self.offset + index;
    }

    pub fn real_len(&self) -> usize {
        self.top
    }
}