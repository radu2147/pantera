#[derive(Debug)]
pub struct Stack<T> {
    elements: Vec<T>
}

impl<T> Stack<T> {
    pub fn init() -> Self {
        Self{
            elements: Vec::<T>::with_capacity(50)
        }
    }
    pub fn push(&mut self, el: T) {
        self.elements.push(el);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    pub fn set(&mut self, index: usize, el: T) {
        self.elements[index] = el;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}