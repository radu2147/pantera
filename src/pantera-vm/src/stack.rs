use std::collections::LinkedList;

#[derive(Debug)]
pub struct Stack<T> {
    elements: LinkedList<T>
}

impl<T> Stack<T> {
    pub fn init() -> Self {
        Self{
            elements: LinkedList::<T>::new()
        }
    }
    pub fn push(&mut self, el: T) {
        self.elements.push_front(el);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
    }
}