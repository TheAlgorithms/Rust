pub struct DSUNode {
    parent: usize,
    size: usize,
}

pub struct DisjointSetUnion {
    nodes: Vec<DSUNode>,
}

// We are using both path compression and union by size
impl DisjointSetUnion {
    // Create n+1 sets [0, n]
    pub fn new(n: usize) -> DisjointSetUnion {
        let mut nodes = Vec::new();
        nodes.reserve_exact(n + 1);
        for i in 0..=n {
            nodes.push(DSUNode { parent: i, size: 1 });
        }
        DisjointSetUnion { nodes }
    }
    pub fn find_set(&mut self, v: usize) -> usize {
        if v == self.nodes[v].parent {
            return v;
        }
        self.nodes[v].parent = self.find_set(self.nodes[v].parent);
        self.nodes[v].parent
    }
    // Returns the new component of the merged sets,
    // or usize::MAX if they were the same.
    pub fn merge(&mut self, u: usize, v: usize) -> usize {
        let mut a = self.find_set(u);
        let mut b = self.find_set(v);
        if a == b {
            return usize::MAX;
        }
        if self.nodes[a].size < self.nodes[b].size {
            std::mem::swap(&mut a, &mut b);
        }
        self.nodes[b].parent = a;
        self.nodes[a].size += self.nodes[b].size;
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_acyclic_graph() {
        let mut dsu = DisjointSetUnion::new(10);
        // Add edges such that vertices 1..=9 are connected
        // and vertex 10 is not connected to the other ones
        let edges: Vec<(usize, usize)> = vec![
            (1, 2), // +
            (2, 1),
            (2, 3), // +
            (1, 3),
            (4, 5), // +
            (7, 8), // +
            (4, 8), // +
            (3, 8), // +
            (1, 9), // +
            (2, 9),
            (3, 9),
            (4, 9),
            (5, 9),
            (6, 9), // +
            (7, 9),
        ];
        let expected_edges: Vec<(usize, usize)> = vec![
            (1, 2),
            (2, 3),
            (4, 5),
            (7, 8),
            (4, 8),
            (3, 8),
            (1, 9),
            (6, 9),
        ];
        let mut added_edges: Vec<(usize, usize)> = Vec::new();
        for (u, v) in edges {
            if dsu.merge(u, v) < usize::MAX {
                added_edges.push((u, v));
            }
            // Now they should be the same
            assert!(dsu.merge(u, v) == usize::MAX);
        }
        assert_eq!(added_edges, expected_edges);
        let comp_1 = dsu.find_set(1);
        for i in 2..=9 {
            assert_eq!(comp_1, dsu.find_set(i));
        }
        assert_ne!(comp_1, dsu.find_set(10));
    }
}
