//! This module provides functionality to find an Eulerian path in a directed graph.
//! An Eulerian path visits every edge exactly once. The algorithm checks if an Eulerian
//! path exists and, if so, constructs and returns the path.

use std::collections::LinkedList;
use std::vec::Vec;

/// Finds the Eulerian path in a directed graph represented by the number of nodes and edges.
///
/// # Arguments
///
/// * `nodes` - The number of nodes in the graph.
/// * `edges` - A vector of tuples representing the directed edges in the graph, where each tuple
///             is of the form `(u, v)` indicating a directed edge from `u` to `v`.
///
/// The function checks if an Eulerian path exists and, if so, constructs and returns one valid path.
///
/// # Returns
///
/// An `Option` containing a vector representing the Eulerian path if it exists, or `None` if no such path exists.
pub fn find_eulerian_path(nodes: usize, edges: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut graph = vec![Vec::new(); nodes];
    for (u, v) in edges {
        graph[u].push(v);
    }

    let mut solver = EulerianPath::new(graph);
    solver.find_eulerian_path()
}

/// Struct representing an Eulerian path in a directed graph.
pub struct EulerianPath {
    nodes: usize,            // Number of nodes
    edges: usize,            // Total number of edges
    in_deg: Vec<usize>,      // In-degrees of nodes
    out_deg: Vec<usize>,     // Out-degrees of nodes
    path: LinkedList<usize>, // Stores the Eulerian path
    graph: Vec<Vec<usize>>,  // Adjacency list
}

impl EulerianPath {
    /// Creates a new instance of `EulerianPath` for the given graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - A directed graph represented as an adjacency list.
    ///
    /// # Returns
    ///
    /// A new `EulerianPath` instance.
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        Self {
            nodes: graph.len(),
            edges: 0,
            in_deg: vec![0; graph.len()],
            out_deg: vec![0; graph.len()],
            path: LinkedList::new(),
            graph,
        }
    }

    /// Finds an Eulerian path if it exists.
    ///
    /// # Returns
    ///
    /// An `Option` containing the Eulerian path as a vector if it exists, or `None` otherwise.
    fn find_eulerian_path(&mut self) -> Option<Vec<usize>> {
        self.init_degrees();

        if !self.has_eulerian_path() {
            return None;
        }

        let start = self.find_start();
        self.dfs(start);

        if self.path.len() != self.edges + 1 {
            return None;
        }

        let mut solution = Vec::with_capacity(self.edges + 1);
        while let Some(node) = self.path.pop_front() {
            solution.push(node);
        }

        Some(solution)
    }

    /// Initializes in-degrees, out-degrees, and counts the total number of edges.
    fn init_degrees(&mut self) {
        for (u, neighbors) in self.graph.iter().enumerate() {
            for &v in neighbors {
                self.in_deg[v] += 1;
                self.out_deg[u] += 1;
                self.edges += 1;
            }
        }
    }

    /// Checks if the graph has an Eulerian path.
    ///
    /// # Returns
    ///
    /// `true` if an Eulerian path exists, `false` otherwise.
    fn has_eulerian_path(&self) -> bool {
        if self.edges == 0 {
            return false;
        }

        let (mut start, mut end) = (0, 0);
        for i in 0..self.nodes {
            let (in_deg, out_deg) = (self.in_deg[i] as isize, self.out_deg[i] as isize);
            match out_deg - in_deg {
                1 => start += 1,
                -1 => end += 1,
                d if d.abs() > 1 => return false,
                _ => (),
            }
        }

        (start == 0 && end == 0) || (start == 1 && end == 1)
    }

    /// Finds the start node for the Eulerian path.
    ///
    /// # Returns
    ///
    /// The index of the start node.
    fn find_start(&self) -> usize {
        for i in 0..self.nodes {
            if self.out_deg[i] > self.in_deg[i] {
                return i;
            }
        }
        (0..self.nodes).find(|&i| self.out_deg[i] > 0).unwrap_or(0)
    }

    /// Depth-first search traversal to construct the Eulerian path.
    ///
    /// # Arguments
    ///
    /// * `u` - The current node being traversed.
    fn dfs(&mut self, u: usize) {
        while self.out_deg[u] > 0 {
            let v = self.graph[u][self.out_deg[u] - 1];
            self.out_deg[u] -= 1;
            self.dfs(v);
        }
        self.path.push_front(u);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, edges, expected) = $test_case;
                    assert_eq!(find_eulerian_path(n, edges), expected);
                }
            )*
        }
    }

    test_cases! {
        test_eulerian_cycle: (
            7,
            vec![
                (1, 2),
                (1, 3),
                (2, 2),
                (2, 4),
                (2, 4),
                (3, 1),
                (3, 2),
                (3, 5),
                (4, 3),
                (4, 6),
                (5, 6),
                (6, 3)
            ],
            Some(vec![1, 3, 5, 6, 3, 2, 4, 3, 1, 2, 2, 4, 6])
        ),
        test_simple_path: (
            5,
            vec![
                (0, 1),
                (1, 2),
                (1, 4),
                (1, 3),
                (2, 1),
                (4, 1)
            ],
            Some(vec![0, 1, 4, 1, 2, 1, 3])
        ),
        test_disconnected_graph: (
            4,
            vec![
                (0, 1),
                (2, 3)
            ],
            None::<Vec<usize>>
        ),
        test_single_cycle: (
            4,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0)
            ],
            Some(vec![0, 1, 2, 3, 0])
        ),
        test_empty_graph: (
            3,
            vec![],
            None::<Vec<usize>>
        ),
        test_unbalanced_path: (
            3,
            vec![
                (0, 1),
                (1, 2),
                (2, 0),
                (0, 2)
            ],
            Some(vec![0, 2, 0, 1, 2])
        ),
        test_no_eulerian_path: (
            3,
            vec![
                (0, 1),
                (0, 2)
            ],
            None::<Vec<usize>>
        ),
        test_complex_eulerian_path: (
            6,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 0),
                (0, 5),
                (5, 0),
                (2, 0)
            ],
            Some(vec![2, 0, 5, 0, 1, 2, 3, 4, 0])
        ),
        test_single_node_self_loop: (
            1,
            vec![(0, 0)],
            Some(vec![0, 0])
        ),
        test_complete_graph: (
            3,
            vec![
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1)
            ],
            Some(vec![0, 2, 1, 2, 0, 1, 0])
        ),
        test_multiple_disconnected_components: (
            6,
            vec![
                (0, 1),
                (2, 3),
                (4, 5)
            ],
            None::<Vec<usize>>
        ),
        test_unbalanced_graph_with_path: (
            4,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 1)
            ],
            Some(vec![0, 1, 2, 3, 1])
        ),
        test_node_with_no_edges: (
            4,
            vec![
                (0, 1),
                (1, 2)
            ],
            Some(vec![0, 1, 2])
        ),
        test_multiple_edges_between_same_nodes: (
            3,
            vec![
                (0, 1),
                (1, 2),
                (1, 2),
                (2, 0)
            ],
            Some(vec![1, 2, 0, 1, 2])
        ),
        test_larger_graph_with_eulerian_path: (
            10,
            vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 8),
                (8, 9),
                (9, 0),
                (1, 6),
                (6, 3),
                (3, 8)
            ],
            Some(vec![1, 6, 3, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8])
        ),
        test_no_edges_multiple_nodes: (
            5,
            vec![],
            None::<Vec<usize>>
        ),
        test_multiple_start_and_end_nodes: (
            4,
            vec![
                (0, 1),
                (1, 2),
                (2, 0),
                (0, 2),
                (1, 3)
            ],
            None::<Vec<usize>>
        ),
        test_single_edge: (
            2,
            vec![(0, 1)],
            Some(vec![0, 1])
        ),
        test_multiple_eulerian_paths: (
            4,
            vec![
                (0, 1),
                (1, 2),
                (2, 0),
                (0, 3),
                (3, 0)
            ],
            Some(vec![0, 3, 0, 1, 2, 0])
        ),
        test_dag_path: (
            4,
            vec![
                (0, 1),
                (1, 2),
                (2, 3)
            ],
            Some(vec![0, 1, 2, 3])
        ),
        test_parallel_edges_case: (
            2,
            vec![
                (0, 1),
                (0, 1),
                (1, 0)
            ],
            Some(vec![0, 1, 0, 1])
        ),
    }
}
