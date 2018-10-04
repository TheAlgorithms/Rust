pub struct Queue<T> {
    data: Vec<T>,
    index: usize,
    front: usize,
    capacity: usize,
}

impl<T> Queue<T> {
    // Returns a new Queue object with given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            index: 0,
            front: 0,
            capacity,
        }
    }

    // Adds the element to the circular queue
    // Once capacity is reached, new items will overwrite old ones.
    pub fn enqueue(&mut self, e: T) {
        if self.len() < self.capacity() {
            self.data.push(e);
        } else {
            self.data[self.index] = e;
        }
        self.index = (self.index + 1) % self.capacity();
    }

    // Returns an Option with first element of the list,
    // if it is empty returns None
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let ele = self.data.swap_remove(self.front as usize);
        if self.len() == 0 {
            self.front = 0
        } else {
            self.front = (self.front + 1) % self.len();
        }
        Some(ele)
    }

    // Returns true if queue is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Returns an Option with the first element, if queue is empty gives None
    // This does not remove the element, just provides a ref.
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        Some(&self.data[self.front])
    }

    // Returns the length of queue
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // Returns the capacity of the queue
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use data_structure::circular_queue::Queue;

    #[test]
    fn empty_circular_queue() {
        let queue = Queue::<i32>::with_capacity(5);
        assert!(queue.is_empty());
        assert_eq!(queue.capacity(), 5);
    }
    #[test]
    fn circular_queue_enqueue() {
        let mut queue: Queue<i32> = Queue::with_capacity(5);
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
    fn circular_queue_dequeue() {
        let mut queue: Queue<i32> = Queue::with_capacity(5);
        assert!(queue.dequeue().is_none());
        queue.enqueue(1);
        assert!(!queue.is_empty());
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        queue.dequeue();
        queue.dequeue();
        assert!(queue.is_empty());
    }
}
