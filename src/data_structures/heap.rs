//! A generic heap data structure.
//!
//! This module provides a `Heap` implementation that can function as either a
//! min-heap or a max-heap. It supports common heap operations such as adding,
//! removing, and iterating over elements. The heap can also be created from
//! an unsorted vector and supports custom comparators for flexible sorting
//! behavior.

use std::{cmp::Ord, slice::Iter};

/// A heap data structure that can be used as a min-heap, max-heap or with
/// custom comparators.
///
/// This struct manages a collection of items where the heap property is maintained.
/// The heap can be configured to order elements based on a provided comparator function,
/// allowing for both min-heap and max-heap functionalities, as well as custom sorting orders.
pub struct Heap<T> {
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    /// Creates a new, empty heap with a custom comparator function.
    ///
    /// # Parameters
    /// - `comparator`: A function that defines the heap's ordering.
    ///
    /// # Returns
    /// A new `Heap` instance.
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            items: vec![],
            comparator,
        }
    }

    /// Creates a heap from a vector and a custom comparator function.
    ///
    /// # Parameters
    /// - `items`: A vector of items to be turned into a heap.
    /// - `comparator`: A function that defines the heap's ordering.
    ///
    /// # Returns
    /// A `Heap` instance with the elements from the provided vector.
    pub fn from_vec(items: Vec<T>, comparator: fn(&T, &T) -> bool) -> Self {
        let mut heap = Self { items, comparator };
        heap.build_heap();
        heap
    }

    /// Constructs the heap from an unsorted vector by applying the heapify process.
    fn build_heap(&mut self) {
        let last_parent_idx = (self.len() / 2).wrapping_sub(1);
        for idx in (0..=last_parent_idx).rev() {
            self.heapify_down(idx);
        }
    }

    /// Returns the number of elements in the heap.
    ///
    /// # Returns
    /// The number of elements in the heap.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Checks if the heap is empty.
    ///
    /// # Returns
    /// `true` if the heap is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds a new element to the heap and maintains the heap property.
    ///
    /// # Parameters
    /// - `value`: The value to add to the heap.
    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.heapify_up(self.len() - 1);
    }

    /// Removes and returns the root element from the heap.
    ///
    /// # Returns
    /// The root element if the heap is not empty, otherwise `None`.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let next = Some(self.items.swap_remove(0));
        if !self.is_empty() {
            self.heapify_down(0);
        }
        next
    }

    /// Returns an iterator over the elements in the heap.
    ///
    /// # Returns
    /// An iterator over the elements in the heap, in their internal order.
    pub fn iter(&self) -> Iter<'_, T> {
        self.items.iter()
    }

    /// Moves an element upwards to restore the heap property.
    ///
    /// # Parameters
    /// - `idx`: The index of the element to heapify up.
    fn heapify_up(&mut self, mut idx: usize) {
        while let Some(pdx) = self.parent_idx(idx) {
            if (self.comparator)(&self.items[idx], &self.items[pdx]) {
                self.items.swap(idx, pdx);
                idx = pdx;
            } else {
                break;
            }
        }
    }

    /// Moves an element downwards to restore the heap property.
    ///
    /// # Parameters
    /// - `idx`: The index of the element to heapify down.
    fn heapify_down(&mut self, mut idx: usize) {
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

            if (self.comparator)(&self.items[cdx], &self.items[idx]) {
                self.items.swap(idx, cdx);
                idx = cdx;
            } else {
                break;
            }
        }
    }

    /// Returns the index of the parent of the element at `idx`.
    ///
    /// # Parameters
    /// - `idx`: The index of the element.
    ///
    /// # Returns
    /// The index of the parent element if it exists, otherwise `None`.
    fn parent_idx(&self, idx: usize) -> Option<usize> {
        if idx > 0 {
            Some((idx - 1) / 2)
        } else {
            None
        }
    }

    /// Checks if the element at `idx` has children.
    ///
    /// # Parameters
    /// - `idx`: The index of the element.
    ///
    /// # Returns
    /// `true` if the element has children, `false` otherwise.
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.len()
    }

    /// Returns the index of the left child of the element at `idx`.
    ///
    /// # Parameters
    /// - `idx`: The index of the element.
    ///
    /// # Returns
    /// The index of the left child.
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    /// Returns the index of the right child of the element at `idx`.
    ///
    /// # Parameters
    /// - `idx`: The index of the element.
    ///
    /// # Returns
    /// The index of the right child.
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
}

impl<T> Heap<T>
where
    T: Ord,
{
    /// Creates a new min-heap.
    ///
    /// # Returns
    /// A new `Heap` instance configured as a min-heap.
    pub fn new_min() -> Heap<T> {
        Self::new(|a, b| a < b)
    }

    /// Creates a new max-heap.
    ///
    /// # Returns
    /// A new `Heap` instance configured as a max-heap.
    pub fn new_max() -> Heap<T> {
        Self::new(|a, b| a > b)
    }

    /// Creates a min-heap from an unsorted vector.
    ///
    /// # Parameters
    /// - `items`: A vector of items to be turned into a min-heap.
    ///
    /// # Returns
    /// A `Heap` instance configured as a min-heap.
    pub fn from_vec_min(items: Vec<T>) -> Heap<T> {
        Self::from_vec(items, |a, b| a < b)
    }

    /// Creates a max-heap from an unsorted vector.
    ///
    /// # Parameters
    /// - `items`: A vector of items to be turned into a max-heap.
    ///
    /// # Returns
    /// A `Heap` instance configured as a max-heap.
    pub fn from_vec_max(items: Vec<T>) -> Heap<T> {
        Self::from_vec(items, |a, b| a > b)
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
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
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
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_iter_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        let mut iter = heap.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_from_vec_min() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut heap = Heap::from_vec_min(vec);
        assert_eq!(heap.len(), 9);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        heap.add(0);
        assert_eq!(heap.pop(), Some(0));
    }

    #[test]
    fn test_from_vec_max() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut heap = Heap::from_vec_max(vec);
        assert_eq!(heap.len(), 9);
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), Some(5));
        heap.add(10);
        assert_eq!(heap.pop(), Some(10));
    }
}
