use std::default::Default;

struct Queue<T>
where
    T: Default,
{
    items: Vec<T>,
}

impl<T> Queue<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.items.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queue() {
        let mut q = Queue::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(6);
        q.enqueue(9);
        assert_eq!(q.len(), 4);
        assert_eq!(q.dequeue(), 1);
        assert_eq!(q.len(), 3);
        assert_eq!(q.dequeue(), 2);
        assert_eq!(q.len(), 2);
        assert_eq!(q.dequeue(), 6);
        assert_eq!(q.len(), 1);
        assert_eq!(q.dequeue(), 9);
        assert_eq!(q.len(), 0);
    }
}