/// A queue structure, put things in the queue and get the oldest items back out first
pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue
    pub fn new() -> Queue<T> {
        Self { items: Vec::new() }
    }

    /// Enqueues an item
    pub fn put(&mut self, item: T) {
        self.items.push(item);
    }

    /// Gets an item from the front of the queue
    pub fn get(&mut self) -> Option<T> {
        if self.len() > 0 {
            Some(self.items.remove(0))
        } else {
            None
        }
    }

    /// Returns the length of the queue
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Rotates the items in the queue
    pub fn rotate(&mut self, times: usize) {
        (0..times).for_each(|_| match self.get() {
            Some(item) => self.put(item),
            None => (),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut queue: Queue<usize> = Queue::new();

        // Returns None when empty
        assert!(queue.get().is_none());

        queue.put(1);
        queue.put(2);
        queue.put(3);

        // Private API
        assert_eq!(queue.items, vec![1, 2, 3]);

        // Rotate once
        queue.rotate(1);
        assert_eq!(queue.items, vec![2, 3, 1]);

        // Rotate twice more, back to original order
        queue.rotate(2);
        assert_eq!(queue.items, vec![1, 2, 3]);

        // Get two items, beginning at the start
        assert_eq!(queue.get().unwrap(), 1);
        assert_eq!(queue.get().unwrap(), 2);

        // Put another item
        queue.put(4);
        assert_eq!(queue.items, vec![3, 4]);

        // Get 3 more items
        assert_eq!(queue.get().unwrap(), 3);
        assert_eq!(queue.get().unwrap(), 4);

        // We should be empty again
        assert!(queue.get().is_none());

        // Put one item
        queue.put(1);

        // Rotating larger than length is a no op
        assert_eq!(queue.items, vec![1]);
        queue.rotate(2);
        assert_eq!(queue.items, vec![1]);
    }
}
