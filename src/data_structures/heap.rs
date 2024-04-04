// Heap data structure
// Takes a closure as a comparator to allow for min-heap, max-heap, and works with custom key functions

use std::{cmp::Ord, slice::Iter};

pub struct Heap<T> {
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            // Add a default in the first spot to offset indexes
            // for the parent/child math to work out.
            // Vecs have to have all the same type so using Default
            // is a way to add an unused item.
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);

        // Heapify Up
        let mut idx = self.len() - 1;
        while let Some(pdx) = self.parent_idx(idx) {
            if (self.comparator)(&self.items[idx], &self.items[pdx]) {
                self.items.swap(idx, pdx);
            }
            idx = pdx;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // This feels like a function built for heap impl :)
        // Removes an item at an index and fills in with the last item
        // of the Vec
        let next = Some(self.items.swap_remove(0));

        if !self.is_empty() {
            // Heapify Down
            let mut idx = 0;
            while self.children_present(idx) {
                let cdx = {
                    if self.right_child_idx(idx) >= self.len() {
                        self.left_child_idx(idx)
                    } else {
                        let ldx = self.left_child_idx(idx);
                        let rdx = self.right_child_idx(idx);
                        if (self.comparator)(&self.items[ldx], &self.items[rdx]) {
                            ldx
                        } else {
                            rdx
                        }
                    }
                };
                if !(self.comparator)(&self.items[idx], &self.items[cdx]) {
                    self.items.swap(idx, cdx);
                }
                idx = cdx;
            }
        }

        next
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }

    fn parent_idx(&self, idx: usize) -> Option<usize> {
        if idx > 0 {
            Some((idx - 1) / 2)
        } else {
            None
        }
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= (self.len() - 1)
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
}

impl<T> Heap<T>
where
    T: Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Heap<T> {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Heap<T> {
        Self::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap: Heap<i32> = Heap::new_max();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        heap.add(1);
        assert_eq!(heap.pop(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(4));
        heap.add(1);
        assert_eq!(heap.pop(), Some(2));
    }

    #[allow(dead_code)]
    struct Point(/* x */ i32, /* y */ i32);

    #[test]
    fn test_key_heap() {
        let mut heap: Heap<Point> = Heap::new(|a, b| a.0 < b.0);
        heap.add(Point(1, 5));
        heap.add(Point(3, 10));
        heap.add(Point(-2, 4));
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.pop().unwrap().0, -2);
        assert_eq!(heap.pop().unwrap().0, 1);
        heap.add(Point(50, 34));
        assert_eq!(heap.pop().unwrap().0, 3);
    }

    #[test]
    fn test_iter_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        // test iterator, which is not in order except the first one.
        let mut iter = heap.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_ne!(iter.next(), None);
        assert_ne!(iter.next(), None);
        assert_ne!(iter.next(), None);
        assert_eq!(iter.next(), None);

        // test the heap after run iterator.
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }
}
