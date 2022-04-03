/*
 Note: We will assume that here tree vertices are numbered from 1 to n.
If a tree is not enumerated that way or its vertices are not represented
using numbers, it can trivially be converted using Depth First Search
manually or by using `src/graph/graph_enumeration.rs`

 Here we implement two different algorithms:
- The online one is implemented using Sparse Table and has O(n.lg(n))
time complexity and memory usage. It answers each query in O(lg(n)).
- The offline algorithm was discovered by Robert Tarjan. At first each
query should be determined and saved. Then, vertices are visited in
Depth First Search order and queries are answered using Disjoint
Set Union algorithm. The time complexity is O(n.alpha(n) + q) and
memory usage is O(n + q), but time complexity can be considered to be O(n + q),
because alpha(n) < 5 for n < 10 ^ 600
 */

use super::DisjointSetUnion;
pub struct LowestCommonAncestorOnline {
    // Make members public to allow the user to fill them themself.
    pub parents_sparse_table: Vec<Vec<usize>>,
    pub height: Vec<usize>,
}

impl LowestCommonAncestorOnline {
    // Should be called once as:
    // fill_sparse_table(tree_root, 0, 0, adjacency_list)
    #[inline]
    fn get_parent(&self, v: usize, i: usize) -> usize {
        self.parents_sparse_table[v][i]
    }
    #[inline]
    fn num_parents(&self, v: usize) -> usize {
        self.parents_sparse_table[v].len()
    }
    pub fn new(num_vertices: usize) -> Self {
        let mut pars = vec![vec![0]; num_vertices + 1];
        pars[0].clear();
        LowestCommonAncestorOnline {
            parents_sparse_table: pars,
            height: vec![0; num_vertices + 1],
        }
    }
    pub fn fill_sparse_table(
        &mut self,
        vertex: usize,
        parent: usize,
        height: usize,
        adj: &[Vec<usize>],
    ) {
        self.parents_sparse_table[vertex][0] = parent;
        self.height[vertex] = height;
        let mut level = 1;
        let mut current_parent = parent;
        while self.num_parents(current_parent) >= level {
            current_parent = self.get_parent(current_parent, level - 1);
            level += 1;
            self.parents_sparse_table[vertex].push(current_parent);
        }
        for &child in adj[vertex].iter() {
            if child == parent {
                // It isn't a child!
                continue;
            }
            self.fill_sparse_table(child, vertex, height + 1, adj);
        }
    }

    pub fn get_ancestor(&self, mut v: usize, mut u: usize) -> usize {
        if self.height[v] < self.height[u] {
            std::mem::swap(&mut v, &mut u);
        }
        // Bring v up to so that it has the same height as u
        let height_diff = self.height[v] - self.height[u];
        for i in 0..63 {
            let bit = 1 << i;
            if bit > height_diff {
                break;
            }
            if height_diff & bit != 0 {
                v = self.get_parent(v, i);
            }
        }
        if u == v {
            return u;
        }
        // `self.num_parents` of u and v should be equal
        for i in (0..self.num_parents(v)).rev() {
            let nv = self.get_parent(v, i);
            let nu = self.get_parent(u, i);
            if nv != nu {
                u = nu;
                v = nv;
            }
        }
        self.get_parent(v, 0)
    }
}

#[derive(Clone, Copy)]
pub struct LCAQuery {
    other: usize,
    query_id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueryAnswer {
    query_id: usize,
    answer: usize,
}

pub struct LowestCommonAncestorOffline {
    pub queries: Vec<Vec<LCAQuery>>,
    dsu: DisjointSetUnion,
    /*
    The LSB of dsu_parent[v] determines whether it was visited or not.
    The rest of the number determines the vertex that represents a
    particular set in DSU.
    */
    dsu_parent: Vec<u64>,
}

impl LowestCommonAncestorOffline {
    pub fn new(num_vertices: usize) -> Self {
        LowestCommonAncestorOffline {
            queries: vec![vec![]; num_vertices + 1],
            dsu: DisjointSetUnion::new(num_vertices),
            dsu_parent: vec![0; num_vertices + 1],
        }
    }
    pub fn add_query(&mut self, u: usize, v: usize, query_id: usize) {
        // We should add this query to both vertices, and it will be answered
        // the second time it is seen in DFS.
        self.queries[u].push(LCAQuery { other: v, query_id });
        if u == v {
            return;
        }
        self.queries[v].push(LCAQuery { other: u, query_id });
    }

    fn calculate_answers(
        &mut self,
        vertex: usize,
        parent: usize,
        adj: &[Vec<usize>],
        answers: &mut Vec<QueryAnswer>,
    ) {
        self.dsu_parent[vertex] = (vertex as u64) << 1;
        for &child in adj[vertex].iter() {
            if child == parent {
                continue;
            }
            self.calculate_answers(child, vertex, adj, answers);
            self.dsu.merge(child, vertex);
            let set = self.dsu.find_set(vertex);
            self.dsu_parent[set] = ((vertex as u64) << 1) | (self.dsu_parent[set] & 1);
        }
        self.dsu_parent[vertex] |= 0b1;
        for &query in self.queries[vertex].iter() {
            if self.dsu_parent[query.other] & 1 != 0 {
                // It has been visited
                answers.push(QueryAnswer {
                    query_id: query.query_id,
                    answer: (self.dsu_parent[self.dsu.find_set(query.other)] >> 1) as usize,
                });
            }
        }
    }
    pub fn answer_queries(&mut self, root: usize, adj: &[Vec<usize>]) -> Vec<QueryAnswer> {
        let mut answers = Vec::new();
        self.calculate_answers(root, 0, adj, &mut answers);
        answers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_binary_tree() {
        let num_verts = 127;
        let mut tree: Vec<Vec<usize>> = vec![vec![]; num_verts + 1];
        for i in 1..=num_verts >> 1 {
            let left_child = i << 1;
            let right_child = left_child + 1;
            tree[i].push(left_child);
            tree[i].push(right_child);
            tree[left_child].push(i);
            tree[right_child].push(i);
        }
        let mut online_answers: Vec<QueryAnswer> = Vec::new();
        let mut online = LowestCommonAncestorOnline::new(num_verts);
        let mut offline = LowestCommonAncestorOffline::new(num_verts);
        let mut query_id = 314; // A random number, doesn't matter
        online.fill_sparse_table(1, 0, 0, &tree);
        for i in 1..=num_verts {
            for j in 1..i {
                // Query every possible pair
                online_answers.push(QueryAnswer {
                    query_id,
                    answer: online.get_ancestor(i, j),
                });
                offline.add_query(i, j, query_id);
                query_id += 1;
            }
        }
        let mut offline_answers = offline.answer_queries(1, &tree);
        offline_answers.sort_unstable_by(|a1, a2| a1.query_id.cmp(&a2.query_id));
        assert_eq!(offline_answers, online_answers);
    }
}
