use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// UnionFind data structure
/// It acts by holding an array of pointers to parents, together with the size of each subset
#[derive(Debug)]
pub struct UnionFind<T: Debug + Eq + Hash> {
    payloads: HashMap<T, usize>, // we are going to manipulate indices to parent, thus `usize`. We need a map to associate a value to its index in the parent links array
    parent_links: Vec<usize>, // holds the relationship between an item and its parent. The root of a set is denoted by parent_links[i] == i
    sizes: Vec<usize>, // holds the size
    count: usize,
}

impl<T: Debug + Eq + Hash> UnionFind<T> {

    /// Creates an empty Union Find structure with capacity n
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::data_structures::UnionFind;
    /// let uf = UnionFind::with_capacity(5);
    /// assert_eq!(0, uf.count())
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parent_links: Vec::with_capacity(capacity),
            sizes: Vec::with_capacity(capacity),
            payloads: HashMap::with_capacity(capacity),
            count: 0
        }
    }

    /// Inserts a new item (disjoint) in the data structure
    pub fn insert(&mut self, item: T) {
        let key = self.payloads.len();
        self.parent_links.push(key);
        self.sizes.push(1);
        self.payloads.insert(item, key);
        self.count += 1;
    }

    pub fn id(&self, value: &T) -> Option<usize> {
        self.payloads.get(value).copied()
    }

    /// Returns the key of an item stored in the data structure or None if it doesn't exist
    fn find(&self, value: &T) -> Option<usize> {
        self.id(value)
            .map(|id| self.find_by_key(id))
    }

    /// Creates a link between value_1 and value_2
    /// returns None if either value_1 or value_2 hasn't been inserted in the data structure first
    /// returns Some(true) if two disjoint sets have been merged
    /// returns Some(false) if both elements already were belonging to the same set
    ///
    /// #_Examples:
    ///
    /// ```
    /// use the_algorithms_rust::data_structures::UnionFind;
    /// let mut uf = UnionFind::with_capacity(2);
    /// uf.insert("A");
    /// uf.insert("B");
    ///
    /// assert_eq!(None, uf.union(&"A", &"C"));
    ///
    /// assert_eq!(2, uf.count());
    /// assert_eq!(Some(true), uf.union(&"A", &"B"));
    /// assert_eq!(1, uf.count());
    ///
    /// assert_eq!(Some(false), uf.union(&"A", &"B"));
    /// ```
    pub fn union(&mut self, item1: &T, item2: &T) -> Option<bool> {
        match (self.find(item1), self.find(item2)) {
            (Some(k1), Some(k2)) => Some(self.union_by_key(k1, k2)),
            _ => None,
        }
    }

    /// Returns the parent of the element given its id
    fn find_by_key(&self, key: usize) -> usize {
        let mut id = key;
        while id != self.parent_links[id] {
            id = self.parent_links[id];
        }
        id
    }

    /// Unions the sets containing id1 and id2
    fn union_by_key(&mut self, key1: usize, key2: usize) -> bool {
        let root1 = self.find_by_key(key1);
        let root2 = self.find_by_key(key2);
        if root1 == root2 {
            return false; // they belong to the same set already, no-op
        }
        // Attach the smaller set to the larger one
        if self.sizes[root1] < self.sizes[root2] {
            self.parent_links[root1] = root2;
            self.sizes[root2] += self.sizes[root1];
        } else {
            self.parent_links[root2] = root1;
            self.sizes[root1] += self.sizes[root2];
        }
        self.count -= 1; // we had 2 disjoint sets, now merged as one
        true
    }

    /// Checks if two items belong to the same set
    ///
    /// #_Examples:
    ///
    /// ```
    /// use the_algorithms_rust::data_structures::UnionFind;
    /// let mut uf = UnionFind::from_iter(["A", "B"]);
    /// assert!(!uf.is_same_set(&"A", &"B"));
    ///
    /// uf.union(&"A", &"B");
    /// assert!(uf.is_same_set(&"A", &"B"));
    ///
    /// assert!(!uf.is_same_set(&"A", &"C"));
    /// ```
    pub fn is_same_set(&self, item1: &T, item2: &T) -> bool {
        matches!((self.find(item1), self.find(item2)), (Some(root1), Some(root2)) if root1 == root2)
    }

    /// Returns the number of disjoint sets
    ///
    /// # Examples
    ///
    /// ```
    /// use the_algorithms_rust::data_structures::UnionFind;
    /// let mut uf = UnionFind::with_capacity(5);
    /// assert_eq!(0, uf.count());
    ///
    /// uf.insert("A");
    /// assert_eq!(1, uf.count());
    ///
    /// uf.insert("B");
    /// assert_eq!(2, uf.count());
    ///
    /// uf.union(&"A", &"B");
    /// assert_eq!(1, uf.count())
    /// ```
    pub fn count(&self) -> usize {
        self.count
    }
}

impl <T: Debug + Eq + Hash> Default for UnionFind<T> {
    fn default() -> Self {
        Self {
            parent_links: Vec::default(),
            sizes: Vec::default(),
            payloads: HashMap::default(),
            count: 0
        }
    }
}

impl<T: Debug + Eq + Hash> FromIterator<T> for UnionFind<T> {
    /// Creates a new UnionFind data structure from an iterable of disjoint elements
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut uf = UnionFind::default();
        for i in iter {
            uf.insert(i);
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
        assert_eq!(uf.find_by_key(0), 0);
        assert_eq!(uf.find_by_key(1), 1);
        assert_eq!(uf.find_by_key(2), 2);
        assert_eq!(uf.find_by_key(3), 3);
        assert_eq!(uf.find_by_key(4), 4);
        assert_eq!(uf.find_by_key(5), 5);
        assert_eq!(uf.find_by_key(6), 6);
        assert_eq!(uf.find_by_key(7), 7);
        assert_eq!(uf.find_by_key(8), 8);
        assert_eq!(uf.find_by_key(9), 9);

        assert_eq!(Some(true), uf.union(&0, &1));
        assert_eq!(Some(true), uf.union(&1, &2));
        assert_eq!(Some(true), uf.union(&2, &3));
        assert_eq!(Some(true), uf.union(&3, &4));
        assert_eq!(Some(true), uf.union(&4, &5));
        assert_eq!(Some(true), uf.union(&5, &6));
        assert_eq!(Some(true), uf.union(&6, &7));
        assert_eq!(Some(true), uf.union(&7, &8));
        assert_eq!(Some(true), uf.union(&8, &9));
        assert_eq!(Some(false), uf.union(&9, &0));

        assert_eq!(1, uf.count());
    }

    #[test]
    fn test_spanning_tree() {
        // Let's imagine the following topology:
        //  A <-> B
        //  B <-> C
        //  A <-> D
        //  E
        //  F <-> G
        // We have 3 disjoint sets: {A, B, C, D}, {E}, {F, G}
        let mut uf = UnionFind::from_iter(["A", "B", "C", "D", "E", "F", "G"]);
        uf.union(&"A", &"B");
        uf.union(&"B", &"C");
        uf.union(&"A", &"D");
        uf.union(&"F", &"G");
        assert_eq!(3, uf.count());
    }
}
