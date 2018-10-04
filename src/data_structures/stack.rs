pub struct Stack<T> {
    data: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            data: vec![]
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn is_empty(&mut self) -> bool {
        self.data.is_empty()
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let last = &self.data[self.len() - 1];
        Some(&last)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use data_structures::stack::Stack;

    #[test]
    fn test_push() {
        let mut stack = Stack::<i32>::new();
        assert!(stack.len() == 0);
        stack.push(1);
        stack.push(2);
        assert!(stack.len() == 2);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::<u8>::new();
        assert!(stack.is_empty());
        stack.push(72);
        assert!(stack.peek() == Some(&72));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::<u32>::new();
        assert!(stack.is_empty());
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert!(stack.len() == 3);
        assert!(stack.pop() == Some(3));
        stack.pop();
        stack.pop();
        assert!(stack.is_empty());
        assert!(stack.pop() == None);
    }
}
