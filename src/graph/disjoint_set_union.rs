//! This module implements the Disjoint Set Union (DSU), also known as Union-Find,
//! which is an efficient data structure for keeping track of a set of elements
//! partitioned into disjoint (non-overlapping) subsets.

/// Represents a node in the Disjoint Set Union (DSU) structure  which
/// keep track of the parent-child relationships in the disjoint sets.
pub struct DSUNode {
    /// The index of the node's parent, or itself if it's the root.
    parent: usize,
    /// The size of the set rooted at this node, used for union by size.
    size: usize,
}

/// Disjoint Set Union (Union-Find) data structure, particularly useful for
/// managing dynamic connectivity problems such as determining
/// if two elements are in the same subset or merging two subsets.
pub struct DisjointSetUnion {
    /// List of DSU nodes where each element's parent and size are tracked.
    nodes: Vec<DSUNode>,
}

impl DisjointSetUnion {
    /// Initializes `n + 1` disjoint sets, each element is its own parent.
    ///
    /// # Parameters
    ///
    /// - `n`: The number of elements to manage (`0` to `n` inclusive).
    ///
    /// # Returns
    ///
    /// A new instance of `DisjointSetUnion` with `n + 1` independent sets.
    pub fn new(num_elements: usize) -> DisjointSetUnion {
        let mut nodes = Vec::with_capacity(num_elements + 1);
        for idx in 0..=num_elements {
            nodes.push(DSUNode {
                parent: idx,
                size: 1,
            });
        }

        DisjointSetUnion { nodes }
    }

    /// Finds the representative (root) of the set containing `element` with path compression.
    ///
    /// Path compression ensures that future queries are faster by directly linking
    /// all nodes in the path to the root.
    ///
    /// # Parameters
    ///
    /// - `element`: The element whose set representative is being found.
    ///
    /// # Returns
    ///
    /// The root representative of the set containing `element`.
    pub fn find_set(&mut self, element: usize) -> usize {
        if element != self.nodes[element].parent {
            self.nodes[element].parent = self.find_set(self.nodes[element].parent);
        }
        self.nodes[element].parent
    }

    /// Merges the sets containing `first_elem` and `sec_elem` using union by size.
    ///
    /// The smaller set is always attached to the root of the larger set to ensure balanced trees.
    ///
    /// # Parameters
    ///
    /// - `first_elem`: The first element whose set is to be merged.
    /// - `sec_elem`: The second element whose set is to be merged.
    ///
    /// # Returns
    ///
    /// The root of the merged set, or `usize::MAX` if both elements are already in the same set.
    pub fn merge(&mut self, first_elem: usize, sec_elem: usize) -> usize {
        let mut first_root = self.find_set(first_elem);
        let mut sec_root = self.find_set(sec_elem);

        if first_root == sec_root {
            // Already in the same set, no merge required
            return usize::MAX;
        }

        // Union by size: attach the smaller tree under the larger tree
        if self.nodes[first_root].size < self.nodes[sec_root].size {
            std::mem::swap(&mut first_root, &mut sec_root);
        }

        self.nodes[sec_root].parent = first_root;
        self.nodes[first_root].size += self.nodes[sec_root].size;

        first_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint_set_union() {
        let mut dsu = DisjointSetUnion::new(10);

        dsu.merge(1, 2);
        dsu.merge(2, 3);
        dsu.merge(1, 9);
        dsu.merge(4, 5);
        dsu.merge(7, 8);
        dsu.merge(4, 8);
        dsu.merge(6, 9);

        assert_eq!(dsu.find_set(1), dsu.find_set(2));
        assert_eq!(dsu.find_set(1), dsu.find_set(3));
        assert_eq!(dsu.find_set(1), dsu.find_set(6));
        assert_eq!(dsu.find_set(1), dsu.find_set(9));

        assert_eq!(dsu.find_set(4), dsu.find_set(5));
        assert_eq!(dsu.find_set(4), dsu.find_set(7));
        assert_eq!(dsu.find_set(4), dsu.find_set(8));

        assert_ne!(dsu.find_set(1), dsu.find_set(10));
        assert_ne!(dsu.find_set(4), dsu.find_set(10));

        dsu.merge(3, 4);

        assert_eq!(dsu.find_set(1), dsu.find_set(2));
        assert_eq!(dsu.find_set(1), dsu.find_set(3));
        assert_eq!(dsu.find_set(1), dsu.find_set(6));
        assert_eq!(dsu.find_set(1), dsu.find_set(9));
        assert_eq!(dsu.find_set(1), dsu.find_set(4));
        assert_eq!(dsu.find_set(1), dsu.find_set(5));
        assert_eq!(dsu.find_set(1), dsu.find_set(7));
        assert_eq!(dsu.find_set(1), dsu.find_set(8));

        assert_ne!(dsu.find_set(1), dsu.find_set(10));

        dsu.merge(10, 1);
        assert_eq!(dsu.find_set(10), dsu.find_set(1));
        assert_eq!(dsu.find_set(10), dsu.find_set(2));
        assert_eq!(dsu.find_set(10), dsu.find_set(3));
        assert_eq!(dsu.find_set(10), dsu.find_set(4));
        assert_eq!(dsu.find_set(10), dsu.find_set(5));
        assert_eq!(dsu.find_set(10), dsu.find_set(6));
        assert_eq!(dsu.find_set(10), dsu.find_set(7));
        assert_eq!(dsu.find_set(10), dsu.find_set(8));
        assert_eq!(dsu.find_set(10), dsu.find_set(9));
    }
}
