//! This module provides a generic implementation of a Trie (prefix tree).
//! A Trie is a tree-like data structure that is commonly used to store sequences of keys
//! (such as strings, integers, or other iterable types) where each node represents one element
//! of the key, and values can be associated with full sequences.

use std::collections::HashMap;
use std::hash::Hash;

/// A single node in the Trie structure, representing a key and an optional value.
#[derive(Debug, Default)]
struct Node<Key: Default, Type: Default> {
    /// A map of children nodes where each key maps to another `Node`.
    children: HashMap<Key, Node<Key, Type>>,
    /// The value associated with this node, if any.
    value: Option<Type>,
}

/// A generic Trie (prefix tree) data structure that allows insertion and lookup
/// based on a sequence of keys.
#[derive(Debug, Default)]
pub struct Trie<Key, Type>
where
    Key: Default + Eq + Hash,
    Type: Default,
{
    /// The root node of the Trie, which does not hold a value itself.
    root: Node<Key, Type>,
}

impl<Key, Type> Trie<Key, Type>
where
    Key: Default + Eq + Hash,
    Type: Default,
{
    /// Creates a new, empty `Trie`.
    ///
    /// # Returns
    /// A `Trie` instance with an empty root node.
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    /// Inserts a value into the Trie, associating it with a sequence of keys.
    ///
    /// # Arguments
    /// - `key`: An iterable sequence of keys (e.g., characters in a string or integers in a vector).
    /// - `value`: The value to associate with the sequence of keys.
    pub fn insert(&mut self, key: impl IntoIterator<Item = Key>, value: Type)
    where
        Key: Eq + Hash,
    {
        let mut node = &mut self.root;
        for c in key {
            node = node.children.entry(c).or_default();
        }
        node.value = Some(value);
    }

    /// Retrieves a reference to the value associated with a sequence of keys, if it exists.
    ///
    /// # Arguments
    /// - `key`: An iterable sequence of keys (e.g., characters in a string or integers in a vector).
    ///
    /// # Returns
    /// An `Option` containing a reference to the value if the sequence of keys exists in the Trie,
    /// or `None` if it does not.
    pub fn get(&self, key: impl IntoIterator<Item = Key>) -> Option<&Type>
    where
        Key: Eq + Hash,
    {
        let mut node = &self.root;
        for c in key {
            node = node.children.get(&c)?;
        }
        node.value.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_and_retrieval_with_strings() {
        let mut trie = Trie::new();

        trie.insert("foo".chars(), 1);
        assert_eq!(trie.get("foo".chars()), Some(&1));
        trie.insert("foobar".chars(), 2);
        assert_eq!(trie.get("foobar".chars()), Some(&2));
        assert_eq!(trie.get("foo".chars()), Some(&1));
        trie.insert("bar".chars(), 3);
        assert_eq!(trie.get("bar".chars()), Some(&3));
        assert_eq!(trie.get("baz".chars()), None);
        assert_eq!(trie.get("foobarbaz".chars()), None);
    }

    #[test]
    fn test_insertion_and_retrieval_with_integers() {
        let mut trie = Trie::new();

        trie.insert(vec![1, 2, 3], 1);
        assert_eq!(trie.get(vec![1, 2, 3]), Some(&1));
        trie.insert(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(trie.get(vec![1, 2, 3, 4, 5]), Some(&2));
        assert_eq!(trie.get(vec![1, 2, 3]), Some(&1));
        trie.insert(vec![10, 20, 30], 3);
        assert_eq!(trie.get(vec![10, 20, 30]), Some(&3));
        assert_eq!(trie.get(vec![4, 5, 6]), None);
        assert_eq!(trie.get(vec![1, 2, 3, 4, 5, 6]), None);
    }

    #[test]
    fn test_empty_trie() {
        let trie: Trie<char, i32> = Trie::new();

        assert_eq!(trie.get("foo".chars()), None);
        assert_eq!(trie.get("".chars()), None);
    }

    #[test]
    fn test_insert_empty_key() {
        let mut trie: Trie<char, i32> = Trie::new();

        trie.insert("".chars(), 42);
        assert_eq!(trie.get("".chars()), Some(&42));
        assert_eq!(trie.get("foo".chars()), None);
    }

    #[test]
    fn test_overlapping_keys() {
        let mut trie = Trie::new();

        trie.insert("car".chars(), 1);
        trie.insert("cart".chars(), 2);
        trie.insert("carter".chars(), 3);
        assert_eq!(trie.get("car".chars()), Some(&1));
        assert_eq!(trie.get("cart".chars()), Some(&2));
        assert_eq!(trie.get("carter".chars()), Some(&3));
        assert_eq!(trie.get("care".chars()), None);
    }

    #[test]
    fn test_partial_match() {
        let mut trie = Trie::new();

        trie.insert("apple".chars(), 10);
        assert_eq!(trie.get("app".chars()), None);
        assert_eq!(trie.get("appl".chars()), None);
        assert_eq!(trie.get("apple".chars()), Some(&10));
        assert_eq!(trie.get("applepie".chars()), None);
    }
}
