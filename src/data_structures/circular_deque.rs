use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq)]
pub struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

pub struct CircularDeque {
    head: Option<Rc<RefCell<Node>>>,
    rear: Option<Rc<RefCell<Node>>>,
    capacity: i32,
    size: i32,
}

impl CircularDeque {
    pub fn new(k: i32) -> Self {
        CircularDeque {
            head: None,
            rear: None,
            capacity: k,
            size: 0,
        }
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let node = Node::new(value);

        if self.is_empty() {
            self.head = Some(node.clone());
            self.rear = Some(node);
        } else {
            node.borrow_mut().next = self.head.take();
            self.head = Some(node);
        }

        self.size += 1;
        true
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let option_node = Some(Node::new(value));

        if self.is_empty() {
            self.head.clone_from(&option_node);
            self.rear = option_node;
        } else {
            let rear = self.rear.as_ref().unwrap();
            rear.borrow_mut().next.clone_from(&option_node);

            self.rear = option_node;
        }

        self.size += 1;
        true
    }

    pub fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        } else if self.is_head_rear_equal() {
            self.rear = None;
            self.head = None;
        } else {
            let head = self.head.take().unwrap();
            self.head.clone_from(&head.borrow().next);
        }

        self.size -= 1;
        true
    }

    pub fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        } else if self.is_head_rear_equal() {
            self.rear = None;
            self.head = None;
        } else {
            let mut curr = self.head.clone();

            while let Some(node) = curr {
                let next_node = node.borrow().next.clone();
                if let Some(next_node) = &next_node {
                    if Some(next_node) == self.rear.as_ref() {
                        node.borrow_mut().next = None;
                        self.rear = Some(node.clone());
                        break;
                    }
                }

                curr = next_node;
            }
        }

        self.size -= 1;
        true
    }

    pub fn get_front(&self) -> i32 {
        if let Some(head) = &self.head {
            head.borrow().val
        } else {
            -1
        }
    }

    pub fn get_rear(&self) -> i32 {
        if let Some(rear) = &self.rear {
            rear.borrow().val
        } else {
            -1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.capacity == self.size
    }

    pub fn is_head_rear_equal(&self) -> bool {
        self.head == self.rear
    }
}

#[cfg(test)]
mod tests {
    use super::CircularDeque;

    #[test]
    fn test_overall() {
        let mut cq = CircularDeque::new(5);

        cq.insert_front(7);
        cq.insert_last(0);
        cq.insert_last(3);
        assert_eq!(cq.get_front(), 7);
        cq.insert_front(9);
        assert_eq!(cq.get_front(), 9);
        assert_eq!(cq.get_rear(), 3);
        cq.delete_last();
        assert_eq!(cq.get_rear(), 0);
        cq.delete_front();
        assert_eq!(cq.get_front(), 7);
        cq.delete_front();
        assert!(cq.is_head_rear_equal());
        cq.delete_last();
        assert!(cq.is_empty())
    }

    #[test]
    fn test_empty_and_full() {
        let mut cq = CircularDeque::new(3);

        assert!(cq.is_empty());
        assert!(!cq.is_full());

        cq.insert_last(1);
        cq.insert_last(2);
        cq.insert_last(3);

        assert!(!cq.is_empty());
        assert!(cq.is_full());
    }

    #[test]
    fn test_insertion_deletion() {
        let mut cq = CircularDeque::new(3);

        cq.insert_front(1);
        cq.insert_front(2);
        cq.insert_last(3);

        assert_eq!(cq.get_front(), 2);
        assert_eq!(cq.get_rear(), 3);

        cq.delete_front();
        assert_eq!(cq.get_front(), 1);

        cq.delete_last();
        assert_eq!(cq.get_rear(), 1);

        cq.insert_last(4);
        assert_eq!(cq.get_rear(), 4);
    }

    #[test]
    fn test_edge_cases() {
        let mut cq = CircularDeque::new(2);

        assert_eq!(cq.get_front(), -1);
        assert_eq!(cq.get_rear(), -1);

        cq.insert_last(1);
        cq.insert_last(2);

        assert_eq!(cq.get_front(), 1);
        assert_eq!(cq.get_rear(), 2);

        cq.delete_front();
        assert_eq!(cq.get_front(), 2);

        cq.delete_last();
        assert!(cq.is_empty());

        cq.insert_front(3);
        assert_eq!(cq.get_front(), 3);
        assert_eq!(cq.get_rear(), 3);
    }
}
