//! This module is about a queue implemented using a singly linked list
//! 
//! The queue follows FIFO (First-In First-Out) principle
//! The [enqueue] method's time complexity is O(1)
//! The [dequeue] method's time complexity is O(1)
//! The [insert] method's time complexity is O(n)
//! The [delete] method's time complexity is O(n)
//! The [peek_front] method's time complexity is O(1)
//! The [peek_back] method's time complexity is O(1)
//! The [len] method's time complexity is O(1)
//! The [is_empty] method's time complexity is O(1)
//!
//! I implemented Iterator, Default and Debug trait for our LinkedListQueue data structure
//!

use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Box<Self> {
        Box::new(Self {
            element,
            next: None,
        })
    }
}

/// LinkedListQueue Implementation using Singly Linked List logic
#[derive(Clone)]
pub struct LinkedListQueue<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    // Act like we own the boxes or nodes, since we are gonna construct and mutate them
    _marker: PhantomData<Box<Node<T>>>,
}

// Implementing default for our LinkedListQueue
impl<T> Default for LinkedListQueue<T> {
    fn default() -> Self {
        LinkedListQueue::new()
    }
}

// Implement iterator for the queue
pub struct LinkedListQueueIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
    _marker: PhantomData<&'a T>,
}

// Implementing Drop for LinkedListQueue
impl<T> Drop for LinkedListQueue<T> {
    fn drop(&mut self) {
        // Dequeue the queue until its empty
        while self.dequeue().is_some() {}
    }
}

// Debug implementation for our LinkedListQueue
impl<T> Debug for LinkedListQueue<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write as _;

        let mut output = String::from("LinkedListQueue ( elements: [");

        for elem in self.iter() {
            let _ = write!(output, " {elem:?} ").is_ok();
        }

        let _ = write!(output, "], length: {} )", self.len()).is_ok();

        write!(f, "{output}")
    }
}

// LinkedListQueueIterator implementation
impl<'a, T> Iterator for LinkedListQueueIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // Initially, current is set to the current tail,
        // It will walk the tail when a user calls next
        self.current.as_ref().map(|node| {
            self.current = &node.next;

            &node.element
        })
    }
}

// Implementation for the queue
impl<T> LinkedListQueue<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    // Iter method, will enably us to iterate through our queue
    pub fn iter(&self) -> LinkedListQueueIter<'_, T> {
        LinkedListQueueIter {
            current: &self.tail,
            _marker: PhantomData,
        }
    }

    /// The enqueue method, more of like the `push_back` of a linked list
    pub fn enqueue(&mut self, element: T)
    where
        T: Clone,
    {
        // We create a new node
        let mut new_node = Node::new(element);
        // We make new_node's next to point to the old tail
        new_node.next = self.tail.take();

        // Here, we are setting the old_tail's next to point to the new pointer
        if let Some(old_tail) = &mut self.tail {
            old_tail.next = Some(Box::clone(&new_node));
        }

        // Making the new_node the tail
        self.tail = Some(new_node.clone());

        // If the list is empty, we first assign the new_node to the head before the tail
        if self.head.is_none() {
            self.head = Some(Box::clone(&new_node));
        }

        // Increment the length
        self.length += 1;
    }

    /// The `dequeue` method, more of like the `pop_front` method of a singly linked list
    pub fn dequeue(&mut self) -> Option<T> {
        // We take the old head, and get next pointer the old head had been pointing to, we get that node,
        // making it the new head
        let result = self.tail.take().map(|mut old_tail| {
            if let Some(next_node) = old_tail.next.take() {
                self.tail = Some(next_node);
            }

            if self.tail.is_none() {
                self.head = None;

                // When the head is popped, the element is none, meaning
                // it will not decrement length
                // Check whether the length is zero, then the list is empty, so it will skip, else
                // decrement 1 from the length
                if !self.is_empty() {
                    self.length -= 1;
                }
            }

            old_tail.element
        });

        // If the list wasn't empty (or popped a node), decrement the length
        // SAFETY: this will prevent the length from going below usize::MIN, because
        // a user may try to dequeue an empty queue, which would lead to negatives
        // which are not supported by the usize
        if result.is_some() && !self.is_empty() {
            self.length -= 1;
        }

        result
    }

    /// Reference to the first element in the queue
    pub fn peek_front(&self) -> Option<&T> {
        self.tail.as_ref().map(|tail| &tail.element)
    }

    // Reference to value at the end of the queue
    pub fn peek_back(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.element)
    }

    // Get element by index from the queue
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut counter = 0;
        // If index == 0, it returns the first element from the queue, using the peek_front
        if index == 0 {
            return self.peek_front();

        // if index is the last, then returns last element using peek_back
        } else if index == (self.len() - 1) {
            return self.peek_back();

            // Else, returns none, if index is out of bounds
        } else if index > (self.len() - 1) {
            return None;
        }

        let mut _current = &self.head;
        let mut get_node: Option<&T> = None;
        // If the node was not got we also index through the tail
        if get_node.is_none() {
            // Setting current to now be the tail
            _current = &self.tail;
            // And also reset counter to 0, because the head will have atmost 1 element
            counter += 1;

            // We traverse to the node to get from the queue
            while let Some(node) = &_current {
                // If the selected index matches the pointer, then set get_node
                // to node at that index
                if counter == index {
                    get_node = Some(&node.element);
                    // Break the loop after getting the element at given index
                    break;
                }

                // Increment counter
                counter += 1;

                _current = &node.next;
            }
        }

        get_node
    }

    /// Insert element at nth position in the queue
    pub fn insert(&mut self, index: usize, element: T)
    where
        T: Clone,
    {
        // Initialize a new node
        let mut new_node = Node::new(element.clone());

        // If the index is greater the last index, then panic
        if self.len() - 1 < index {
            panic!("Trying to insert element to index out of bounds")
        }

        // If the length is zero, then just insert at the tail
        if self.is_empty() {
            self.tail = Some(Box::clone(&new_node));
        } else {
            // If length is greater than zero, we assign current to zero, initially
            let mut current = self.tail.as_mut().unwrap();

            // Create a for loop to end at the index selected by user, then assign
            // the node at that index to current
            // I made it (index - 1) so that it gets the previous node instead of the exact
            // node inorder for current.next to point to the exact node, when it reaches the node
            for _ in 0..index - 1 {
                current = current.next.as_mut().unwrap();
            }

            // We set the new_node's next to be current next node
            new_node.next.clone_from(&current.next);

            // Increment the length
            self.length += 1;
            // Then we set the current's next node to point to the new_node
            current.next = Some(new_node);
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        // Index out of bounds
        if index >= self.length {
            return None;
        }

        if index == 0 {
            // Deleting the head (equivalent to dequeue)
            self.dequeue()
        } else {
            let mut current = self.tail.as_mut()?;

            // Traverse to the node just before the one to delete
            for _ in 0..index - 1 {
                current = current.next.as_mut()?;
            }

            // The node to delete is current.next
            let mut to_delete = current.next.take()?; // Take ownership of the node to delete

            // Re-link the current node to skip over the deleted node
            current.next = to_delete.next.take();

            // If the deleted node was the last node, update the tail pointer
            if current.next.is_none() {
                // If there is no next node, set tail to the current node
                self.tail = Some(current.clone());
            }

            // Decrease the queue length
            self.length -= 1;

            // Return the deleted element
            Some(to_delete.element)
        }
    }

    /// Empty the queue
    pub fn drain(&mut self) {
        while self.dequeue().is_some() {}
    }

    /// Gets the length of the queue
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check whether the queue is empty or not
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

/// The queue implementation tests
#[cfg(test)]
mod tests {
    use super::LinkedListQueue;

    #[test]
    fn test_enqueue() {
        // Creating a new queue
        let mut queue = LinkedListQueue::<i32>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        println!("{:?}", queue.len());

        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_dequeue() {
        let mut queue = LinkedListQueue::<i32>::new();
        // Enqueue a couple of values
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);

        // Then dequeue some values
        queue.dequeue();
        queue.dequeue();

        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_queue_length() {
        let mut queue = LinkedListQueue::new();

        // Enqueue a couple of elements
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_peek_front() {
        let mut queue = LinkedListQueue::default();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(Some(&3), queue.peek_front());
    }

    #[test]
    fn peek_back() {
        let mut queue = LinkedListQueue::default();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(Some(&1), queue.peek_back());
    }

    #[test]
    fn test_get_from_queue() {
        let mut queue = LinkedListQueue::new();

        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(5);

        let result = queue.get(1);
        println!("{result:#?}", );

        assert!(result.is_some());
    }

    #[test]
    fn test_queue_insert() {
        let mut queue = LinkedListQueue::default();

        queue.enqueue(1);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.insert(2, 2);

        assert_eq!(queue.len(), 4);
    }

    #[test]
    fn test_queue_delete() {
        let mut queue = LinkedListQueue::default();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        queue.delete(1);

        assert_eq!(queue.len(), 2);
        // Whether to see whether an option of variant Some is returned
        assert!(queue.delete(1).is_some());
    }
}
