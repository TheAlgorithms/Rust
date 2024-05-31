//! A Union-Find (Disjoint Set) data structure implementation in Rust.
//!
//! The Union-Find data structure keeps track of elements partitioned into
//! disjoint (non-overlapping) sets.
//! It provides near-constant-time operations to add new sets, to find the
//! representative of a set, and to merge sets.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct UnionFind<T: Debug + Eq + Hash> {
    payloads: HashMap<T, usize>, // Maps values to their indices in the parent_links array.
    parent_links: Vec<usize>,    // Holds the parent pointers; root elements are their own parents.
    sizes: Vec<usize>,           // Holds the sizes of the sets.
    count: usize,                // Number of disjoint sets.
}

impl<T: Debug + Eq + Hash> UnionFind<T> {
    /// Creates an empty Union-Find structure with a specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parent_links: Vec::with_capacity(capacity),
            sizes: Vec::with_capacity(capacity),
            payloads: HashMap::with_capacity(capacity),
            count: 0,
        }
    }

    /// Inserts a new item (disjoint set) into the data structure.
    pub fn insert(&mut self, item: T) {
        let key = self.payloads.len();
        self.parent_links.push(key);
        self.sizes.push(1);
        self.payloads.insert(item, key);
        self.count += 1;
    }

    /// Returns the root index of the set containing the given value, or `None` if it doesn't exist.
    pub fn find(&mut self, value: &T) -> Option<usize> {
        self.payloads
            .get(value)
            .copied()
            .map(|key| self.find_by_key(key))
    }

    /// Unites the sets containing the two given values. Returns:
    /// - `None` if either value hasn't been inserted,
    /// - `Some(true)` if two disjoint sets have been merged,
    /// - `Some(false)` if both elements were already in the same set.
    pub fn union(&mut self, first_item: &T, sec_item: &T) -> Option<bool> {
        let (first_root, sec_root) = (self.find(first_item), self.find(sec_item));
        match (first_root, sec_root) {
            (Some(first_root), Some(sec_root)) => Some(self.union_by_key(first_root, sec_root)),
            _ => None,
        }
    }

    /// Finds the root of the set containing the element with the given index.
    fn find_by_key(&mut self, key: usize) -> usize {
        if self.parent_links[key] != key {
            self.parent_links[key] = self.find_by_key(self.parent_links[key]);
        }
        self.parent_links[key]
    }

    /// Unites the sets containing the two elements identified by their indices.
    fn union_by_key(&mut self, first_key: usize, sec_key: usize) -> bool {
        let (first_root, sec_root) = (self.find_by_key(first_key), self.find_by_key(sec_key));

        if first_root == sec_root {
            return false;
        }

        match self.sizes[first_root].cmp(&self.sizes[sec_root]) {
            Ordering::Less => {
                self.parent_links[first_root] = sec_root;
                self.sizes[sec_root] += self.sizes[first_root];
            }
            _ => {
                self.parent_links[sec_root] = first_root;
                self.sizes[first_root] += self.sizes[sec_root];
            }
        }

        self.count -= 1;
        true
    }

    /// Checks if two items belong to the same set.
    pub fn is_same_set(&mut self, first_item: &T, sec_item: &T) -> bool {
        matches!((self.find(first_item), self.find(sec_item)), (Some(first_root), Some(sec_root)) if first_root == sec_root)
    }

    /// Returns the number of disjoint sets.
    pub fn count(&self) -> usize {
        self.count
    }
}

impl<T: Debug + Eq + Hash> Default for UnionFind<T> {
    fn default() -> Self {
        Self {
            parent_links: Vec::default(),
            sizes: Vec::default(),
            payloads: HashMap::default(),
            count: 0,
        }
    }
}

impl<T: Debug + Eq + Hash> FromIterator<T> for UnionFind<T> {
    /// Creates a new UnionFind data structure from an iterable of disjoint elements.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut uf = UnionFind::default();
        for item in iter {
            uf.insert(item);
        }
        uf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::from_iter(0..10);
        assert_eq!(uf.find(&0), Some(0));
        assert_eq!(uf.find(&1), Some(1));
        assert_eq!(uf.find(&2), Some(2));
        assert_eq!(uf.find(&3), Some(3));
        assert_eq!(uf.find(&4), Some(4));
        assert_eq!(uf.find(&5), Some(5));
        assert_eq!(uf.find(&6), Some(6));
        assert_eq!(uf.find(&7), Some(7));
        assert_eq!(uf.find(&8), Some(8));
        assert_eq!(uf.find(&9), Some(9));

        assert!(!uf.is_same_set(&0, &1));
        assert!(!uf.is_same_set(&2, &9));
        assert_eq!(uf.count(), 10);

        assert_eq!(uf.union(&0, &1), Some(true));
        assert_eq!(uf.union(&1, &2), Some(true));
        assert_eq!(uf.union(&2, &3), Some(true));
        assert_eq!(uf.union(&0, &2), Some(false));
        assert_eq!(uf.union(&4, &5), Some(true));
        assert_eq!(uf.union(&5, &6), Some(true));
        assert_eq!(uf.union(&6, &7), Some(true));
        assert_eq!(uf.union(&7, &8), Some(true));
        assert_eq!(uf.union(&8, &9), Some(true));
        assert_eq!(uf.union(&7, &9), Some(false));

        assert_ne!(uf.find(&0), uf.find(&9));
        assert_eq!(uf.find(&0), uf.find(&3));
        assert_eq!(uf.find(&4), uf.find(&9));
        assert!(uf.is_same_set(&0, &3));
        assert!(uf.is_same_set(&4, &9));
        assert!(!uf.is_same_set(&0, &9));
        assert_eq!(uf.count(), 2);

        assert_eq!(Some(true), uf.union(&3, &4));
        assert_eq!(uf.find(&0), uf.find(&9));
        assert_eq!(uf.count(), 1);
        assert!(uf.is_same_set(&0, &9));

        assert_eq!(None, uf.union(&0, &11));
    }

    #[test]
    fn test_spanning_tree() {
        let mut uf = UnionFind::from_iter(["A", "B", "C", "D", "E", "F", "G"]);
        uf.union(&"A", &"B");
        uf.union(&"B", &"C");
        uf.union(&"A", &"D");
        uf.union(&"F", &"G");

        assert_eq!(None, uf.union(&"A", &"W"));

        assert_eq!(uf.find(&"A"), uf.find(&"B"));
        assert_eq!(uf.find(&"A"), uf.find(&"C"));
        assert_eq!(uf.find(&"B"), uf.find(&"D"));
        assert_ne!(uf.find(&"A"), uf.find(&"E"));
        assert_ne!(uf.find(&"A"), uf.find(&"F"));
        assert_eq!(uf.find(&"G"), uf.find(&"F"));
        assert_ne!(uf.find(&"G"), uf.find(&"E"));

        assert!(uf.is_same_set(&"A", &"B"));
        assert!(uf.is_same_set(&"A", &"C"));
        assert!(uf.is_same_set(&"B", &"D"));
        assert!(!uf.is_same_set(&"B", &"F"));
        assert!(!uf.is_same_set(&"E", &"A"));
        assert!(!uf.is_same_set(&"E", &"G"));
        assert_eq!(uf.count(), 3);
    }

    #[test]
    fn test_with_capacity() {
        let mut uf: UnionFind<i32> = UnionFind::with_capacity(5);
        uf.insert(0);
        uf.insert(1);
        uf.insert(2);
        uf.insert(3);
        uf.insert(4);

        assert_eq!(uf.count(), 5);

        assert_eq!(uf.union(&0, &1), Some(true));
        assert!(uf.is_same_set(&0, &1));
        assert_eq!(uf.count(), 4);

        assert_eq!(uf.union(&2, &3), Some(true));
        assert!(uf.is_same_set(&2, &3));
        assert_eq!(uf.count(), 3);

        assert_eq!(uf.union(&0, &2), Some(true));
        assert!(uf.is_same_set(&0, &1));
        assert!(uf.is_same_set(&2, &3));
        assert!(uf.is_same_set(&0, &3));
        assert_eq!(uf.count(), 2);

        assert_eq!(None, uf.union(&0, &10));
    }
}
