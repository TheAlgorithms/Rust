use std::collections::LinkedList;

pub struct HashTable<K, V> {
    elements: Vec<LinkedList<(K, V)>>,
    count: usize,
}

impl<K: Hashable + std::cmp::PartialEq, V> Default for HashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl<K: Hashable + std::cmp::PartialEq, V> HashTable<K, V> {
    pub fn new() -> HashTable<K, V> {
        let initial_capacity = 3000;
        let mut elements = Vec::with_capacity(initial_capacity);

        for _ in 0..initial_capacity {
            elements.push(LinkedList::new());
        }

        HashTable { elements, count: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.count >= self.elements.len() * 3 / 4 {
            self.resize();
        }
        let index = key.hash() % self.elements.len();
        self.elements[index].push_back((key, value));
        self.count += 1;
    }

    pub fn search(&self, key: K) -> Option<&V> {
        let index = key.hash() % self.elements.len();
        self.elements[index]
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    fn resize(&mut self) {
        let new_size = self.elements.len() * 2;
        let mut new_elements = Vec::with_capacity(new_size);

        for _ in 0..new_size {
            new_elements.push(LinkedList::new());
        }

        for old_list in self.elements.drain(..) {
            for (key, value) in old_list {
                let new_index = key.hash() % new_size;
                new_elements[new_index].push_back((key, value));
            }
        }

        self.elements = new_elements;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestKey(usize);

    impl Hashable for TestKey {
        fn hash(&self) -> usize {
            self.0
        }
    }

    #[test]
    fn test_insert_and_search() {
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(1));

        assert_eq!(result, Some(&TestKey(10)));
    }

    #[test]
    fn test_resize() {
        let mut hash_table = HashTable::new();
        let initial_capacity = hash_table.elements.capacity();

        for i in 0..initial_capacity * 3 / 4 + 1 {
            hash_table.insert(TestKey(i), TestKey(i + 10));
        }

        assert!(hash_table.elements.capacity() > initial_capacity);
    }

    #[test]
    fn test_search_nonexistent() {
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(2));

        assert_eq!(result, None);
    }

    #[test]
    fn test_multiple_inserts_and_searches() {
        let mut hash_table = HashTable::new();
        for i in 0..10 {
            hash_table.insert(TestKey(i), TestKey(i + 100));
        }

        for i in 0..10 {
            let result = hash_table.search(TestKey(i));
            assert_eq!(result, Some(&TestKey(i + 100)));
        }
    }

    #[test]
    fn test_not_overwrite_existing_key() {
        let mut hash_table = HashTable::new();
        hash_table.insert(TestKey(1), TestKey(100));
        hash_table.insert(TestKey(1), TestKey(200));

        let result = hash_table.search(TestKey(1));
        assert_eq!(result, Some(&TestKey(100)));
    }

    #[test]
    fn test_empty_search() {
        let hash_table: HashTable<TestKey, TestKey> = HashTable::new();
        let result = hash_table.search(TestKey(1));

        assert_eq!(result, None);
    }
}
