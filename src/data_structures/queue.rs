use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.elements.front()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_enqueue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(64);
        assert!(!queue.is_empty(), "Queue should not be empty after enqueue");
    }

    #[test]
    fn test_dequeue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(32);
        queue.enqueue(64);
        let retrieved_dequeue = queue.dequeue();
        assert_eq!(
            retrieved_dequeue,
            Some(32),
            "Dequeue should return the first element"
        );
    }

    #[test]
    fn test_peek_front() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        let retrieved_peek = queue.peek_front();
        assert_eq!(
            retrieved_peek,
            Some(&8),
            "Peek should return a reference to the first element"
        );
    }

    #[test]
    fn test_size() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(8);
        queue.enqueue(16);
        assert_eq!(
            2,
            queue.len(),
            "Queue length should be equal to the number of enqueued elements"
        );
    }

    #[test]
    fn test_is_empty() {
        let mut queue: Queue<u8> = Queue::new();
        assert!(queue.is_empty(), "Newly created queue should be empty");
        queue.enqueue(8);
        assert!(!queue.is_empty(), "Queue should not be empty after enqueue");
    }
}
