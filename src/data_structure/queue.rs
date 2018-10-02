pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    // Returns a new Queue object
    pub fn new() -> Queue<T> {
        Queue { data: vec![] }
    }

    // Adds the element to the end of the list
    pub fn enqueue(&mut self, e: T) {
        self.data.push(e)
    }

    // Returns an Option with first element of the list,
    // if it is empty returns None
    pub fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        Some(self.data.remove(0))
    }

    // Returns true if queue is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Returns an Option with the first element, if queue is empty gives None
    // This does not remove the element, just displays it.
    pub fn peek(&self) -> Option<&T> {
        if self.data.is_empty() {
            return None;
        }
        let e = &self.data[0];
        Some(e)
    }

    // Returns the length of queue
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use data_structure::queue::Queue;

    #[test]
    fn empty_queue() {
        let queue = Queue::<i32>::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn queue_enqueue() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(1);
        assert!(!queue.is_empty());
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);
        assert_eq!(queue.len(), 5);
        assert_eq!(queue.peek(), Some(&1));
    }

    #[test]
    fn queue_dequeue() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(1);
        assert!(!queue.is_empty());
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        queue.dequeue();
        queue.dequeue();
        assert!(queue.is_empty());
    }
}
