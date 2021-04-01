use std::fmt::{self, Display, Formatter};

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { val: t, next }
    }
}

#[derive(Default)]
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    top: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { head: None, top: 0 }
    }

    /// Push an item to the top of a stack.
    pub fn push(&mut self, value: T) {
        let node = Box::new(Node::new(value, std::mem::replace(&mut self.head, None)));
        self.head = Some(node);
        self.top += 1;
    }

    /// Remove the last item from a stack and returns, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.top -= 1;
                self.head = node.next;
                Some(node.val)
            }
        }
    }

    /// Returns reference of the last item from a stack, or `None` if it is empty.
    pub fn peek(&mut self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        let head = self.head.as_ref().unwrap();
        let value = &head.val;
        Some(value)
    }

    pub fn len(&self) -> usize {
        self.top
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
}

pub struct StackIntoIter<T> {
    stack: Stack<T>,
}

impl<T> Iterator for StackIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIntoIter { stack: self }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.next {
            Some(node) => write!(f, "{}, {}", self.val, node.as_ref()),
            None => write!(f, "{}", self.val),
        }
    }
}

impl<T> Display for Stack<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.head {
            Some(node) => write!(f, "{}", node.as_ref()),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Stack;
    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::<usize>::new();
        stack.push(18);
        stack.push(1);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.peek(), Some(&18));
        assert_eq!(stack.pop(), Some(18));
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn create_string_stack() {
        let mut stack = Stack::<String>::new();
        stack.push("World!".to_string());
        stack.push("Hello".to_string());
        assert_eq!(format!("{:}", stack), "Hello, World!");
    }

    #[test]
    fn test_stack_iter() {
        let mut stack = Stack::<usize>::new();
        stack.push(18);
        stack.push(1);
        stack.push(7);
        stack.push(11);
        assert_eq!(
            stack.into_iter().collect::<Vec<usize>>(),
            vec![11, 7, 1, 18]
        )
    }
}
