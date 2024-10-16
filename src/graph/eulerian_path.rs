//! This module provides functionality to find an Eulerian path in a directed graph.
//! An Eulerian path visits every edge exactly once. The algorithm checks if an Eulerian
//! path exists and, if so, constructs and returns the path.

use std::collections::LinkedList;

/// Finds an Eulerian path in a directed graph.
///
/// # Arguments
///
/// * `node_count` - The number of nodes in the graph.
/// * `edge_list` - A vector of tuples representing directed edges, where each tuple is of the form `(start, end)`.
///
/// # Returns
///
/// An `Option<Vec<usize>>` containing the Eulerian path if it exists; otherwise, `None`.
pub fn find_eulerian_path(node_count: usize, edge_list: Vec<(usize, usize)>) -> Option<Vec<usize>> {
    let mut adjacency_list = vec![Vec::new(); node_count];
    for (start, end) in edge_list {
        adjacency_list[start].push(end);
    }

    let mut eulerian_solver = EulerianPathSolver::new(adjacency_list);
    eulerian_solver.find_path()
}

/// Struct to represent the solver for finding an Eulerian path in a directed graph.
pub struct EulerianPathSolver {
    node_count: usize,
    edge_count: usize,
    in_degrees: Vec<usize>,
    out_degrees: Vec<usize>,
    eulerian_path: LinkedList<usize>,
    adjacency_list: Vec<Vec<usize>>,
}

impl EulerianPathSolver {
    /// Creates a new instance of `EulerianPathSolver`.
    ///
    /// # Arguments
    ///
    /// * `adjacency_list` - The graph represented as an adjacency list.
    ///
    /// # Returns
    ///
    /// A new instance of `EulerianPathSolver`.
    pub fn new(adjacency_list: Vec<Vec<usize>>) -> Self {
        Self {
            node_count: adjacency_list.len(),
            edge_count: 0,
            in_degrees: vec![0; adjacency_list.len()],
            out_degrees: vec![0; adjacency_list.len()],
            eulerian_path: LinkedList::new(),
            adjacency_list,
        }
    }

    /// Find the Eulerian path if it exists.
    ///
    /// # Returns
    ///
    /// An `Option<Vec<usize>>` containing the Eulerian path if found; otherwise, `None`.
    ///
    /// If multiple Eulerian paths exist, the one found will be returned, but it may not be unique.
    fn find_path(&mut self) -> Option<Vec<usize>> {
        self.initialize_degrees();

        if !self.has_eulerian_path() {
            return None;
        }

        let start_node = self.get_start_node();
        self.depth_first_search(start_node);

        if self.eulerian_path.len() != self.edge_count + 1 {
            return None;
        }

        let mut path = Vec::with_capacity(self.edge_count + 1);
        while let Some(node) = self.eulerian_path.pop_front() {
            path.push(node);
        }

        Some(path)
    }

    /// Initializes in-degrees and out-degrees for each node and counts total edges.
    fn initialize_degrees(&mut self) {
        for (start_node, neighbors) in self.adjacency_list.iter().enumerate() {
            for &end_node in neighbors {
                self.in_degrees[end_node] += 1;
                self.out_degrees[start_node] += 1;
                self.edge_count += 1;
            }
        }
    }

    /// Checks if an Eulerian path exists in the graph.
    ///
    /// # Returns
    ///
    /// `true` if an Eulerian path exists; otherwise, `false`.
    fn has_eulerian_path(&self) -> bool {
        if self.edge_count == 0 {
            return false;
        }

        let (mut start_nodes, mut end_nodes) = (0, 0);
        for i in 0..self.node_count {
            let (in_degree, out_degree) =
                (self.in_degrees[i] as isize, self.out_degrees[i] as isize);
            match out_degree - in_degree {
                1 => start_nodes += 1,
                -1 => end_nodes += 1,
                degree_diff if degree_diff.abs() > 1 => return false,
                _ => (),
            }
        }

        (start_nodes == 0 && end_nodes == 0) || (start_nodes == 1 && end_nodes == 1)
    }

    /// Finds the starting node for the Eulerian path.
    ///
    /// # Returns
    ///
    /// The index of the starting node.
    fn get_start_node(&self) -> usize {
        for i in 0..self.node_count {
            if self.out_degrees[i] > self.in_degrees[i] {
                return i;
            }
        }
        (0..self.node_count)
            .find(|&i| self.out_degrees[i] > 0)
            .unwrap_or(0)
    }

    /// Performs depth-first search to construct the Eulerian path.
    ///
    /// # Arguments
    ///
    /// * `curr_node` - The current node being visited in the DFS traversal.
    fn depth_first_search(&mut self, curr_node: usize) {
        while self.out_degrees[curr_node] > 0 {
            let next_node = self.adjacency_list[curr_node][self.out_degrees[curr_node] - 1];
            self.out_degrees[curr_node] -= 1;
            self.depth_first_search(next_node);
        }
        self.eulerian_path.push_front(curr_node);
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
