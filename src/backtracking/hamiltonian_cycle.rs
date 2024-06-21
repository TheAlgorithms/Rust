//! This module provides functionality to find a Hamiltonian cycle in a directed or undirected graph.
//! Source: [Wikipedia](https://en.wikipedia.org/wiki/Hamiltonian_path_problem)

/// Represents potential errors when finding hamiltonian cycle on an adjacency matrix.
#[derive(Debug, PartialEq, Eq)]
pub enum FindHamiltonianCycleError {
    /// Indicates that the adjacency matrix is empty.
    EmptyAdjacencyMatrix,
    /// Indicates that the adjacency matrix is not square.
    ImproperAdjacencyMatrix,
    /// Indicates that the starting vertex is out of bounds.
    StartOutOfBound,
}

/// Represents a graph using an adjacency matrix.
struct Graph {
    /// The adjacency matrix representing the graph.
    adjacency_matrix: Vec<Vec<bool>>,
}

impl Graph {
    /// Creates a new graph with the provided adjacency matrix.
    ///
    /// # Arguments
    ///
    /// * `adjacency_matrix` - A square matrix where each element indicates
    ///                        the presence (`true`) or absence (`false`) of an edge
    ///                        between two vertices.
    ///
    /// # Returns
    ///
    /// A `Result` containing the graph if successful, or an `FindHamiltonianCycleError` if there is an issue with the matrix.
    fn new(adjacency_matrix: Vec<Vec<bool>>) -> Result<Self, FindHamiltonianCycleError> {
        // Check if the adjacency matrix is empty.
        if adjacency_matrix.is_empty() {
            return Err(FindHamiltonianCycleError::EmptyAdjacencyMatrix);
        }

        // Validate that the adjacency matrix is square.
        if adjacency_matrix
            .iter()
            .any(|row| row.len() != adjacency_matrix.len())
        {
            return Err(FindHamiltonianCycleError::ImproperAdjacencyMatrix);
        }

        Ok(Self { adjacency_matrix })
    }

    /// Returns the number of vertices in the graph.
    fn num_vertices(&self) -> usize {
        self.adjacency_matrix.len()
    }

    /// Determines if it is safe to include vertex `v` in the Hamiltonian cycle path.
    ///
    /// # Arguments
    ///
    /// * `v` - The index of the vertex being considered.
    /// * `visited` - A reference to the vector representing the visited vertices.
    /// * `path` - A reference to the current path being explored.
    /// * `pos` - The position of the current vertex being considered.
    ///
    /// # Returns
    ///
    /// `true` if it is safe to include `v` in the path, `false` otherwise.
    fn is_safe(&self, v: usize, visited: &[bool], path: &[Option<usize>], pos: usize) -> bool {
        // Check if the current vertex and the last vertex in the path are adjacent.
        if !self.adjacency_matrix[path[pos - 1].unwrap()][v] {
            return false;
        }

        // Check if the vertex has already been included in the path.
        !visited[v]
    }

    /// Recursively searches for a Hamiltonian cycle.
    ///
    /// This function is called by `find_hamiltonian_cycle`.
    ///
    /// # Arguments
    ///
    /// * `path` - A mutable vector representing the current path being explored.
    /// * `visited` - A mutable vector representing the visited vertices.
    /// * `pos` - The position of the current vertex being considered.
    ///
    /// # Returns
    ///
    /// `true` if a Hamiltonian cycle is found, `false` otherwise.
    fn hamiltonian_cycle_util(
        &self,
        path: &mut [Option<usize>],
        visited: &mut [bool],
        pos: usize,
    ) -> bool {
        if pos == self.num_vertices() {
            // Check if there is an edge from the last included vertex to the first vertex.
            return self.adjacency_matrix[path[pos - 1].unwrap()][path[0].unwrap()];
        }

        for v in 0..self.num_vertices() {
            if self.is_safe(v, visited, path, pos) {
                path[pos] = Some(v);
                visited[v] = true;
                if self.hamiltonian_cycle_util(path, visited, pos + 1) {
                    return true;
                }
                path[pos] = None;
                visited[v] = false;
            }
        }

        false
    }

    /// Attempts to find a Hamiltonian cycle in the graph, starting from the specified vertex.
    ///
    /// A Hamiltonian cycle visits every vertex exactly once and returns to the starting vertex.
    ///
    /// # Note
    /// This implementation may not find all possible Hamiltonian cycles.
    /// It stops as soon as it finds one valid cycle. If multiple Hamiltonian cycles exist,
    /// only one will be returned.
    ///
    /// # Returns
    ///
    /// `Ok(Some(path))` if a Hamiltonian cycle is found, where `path` is a vector
    /// containing the indices of vertices in the cycle, starting and ending with the same vertex.
    ///
    /// `Ok(None)` if no Hamiltonian cycle exists.
    fn find_hamiltonian_cycle(
        &self,
        start_vertex: usize,
    ) -> Result<Option<Vec<usize>>, FindHamiltonianCycleError> {
        // Validate the start vertex.
        if start_vertex >= self.num_vertices() {
            return Err(FindHamiltonianCycleError::StartOutOfBound);
        }

        // Initialize the path.
        let mut path = vec![None; self.num_vertices()];
        // Start at the specified vertex.
        path[0] = Some(start_vertex);

        // Initialize the visited vector.
        let mut visited = vec![false; self.num_vertices()];
        visited[start_vertex] = true;

        if self.hamiltonian_cycle_util(&mut path, &mut visited, 1) {
            // Complete the cycle by returning to the starting vertex.
            path.push(Some(start_vertex));
            Ok(Some(path.into_iter().map(Option::unwrap).collect()))
        } else {
            Ok(None)
        }
    }
}

/// Attempts to find a Hamiltonian cycle in a graph represented by an adjacency matrix, starting from a specified vertex.
pub fn find_hamiltonian_cycle(
    adjacency_matrix: Vec<Vec<bool>>,
    start_vertex: usize,
) -> Result<Option<Vec<usize>>, FindHamiltonianCycleError> {
    Graph::new(adjacency_matrix)?.find_hamiltonian_cycle(start_vertex)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hamiltonian_cycle_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (adjacency_matrix, start_vertex, expected) = $test_case;
                    let result = find_hamiltonian_cycle(adjacency_matrix, start_vertex);
                    assert_eq!(result, expected);
                }
            )*
        };
    }

    hamiltonian_cycle_tests! {
        test_complete_graph: (
            vec![
                vec![false, true, true, true],
                vec![true, false, true, true],
                vec![true, true, false, true],
                vec![true, true, true, false],
            ],
            0,
            Ok(Some(vec![0, 1, 2, 3, 0]))
        ),
        test_directed_graph_with_cycle: (
            vec![
                vec![false, true, false, false, false],
                vec![false, false, true, true, false],
                vec![true, false, false, true, true],
                vec![false, false, true, false, true],
                vec![true, true, false, false, false],
            ],
            2,
            Ok(Some(vec![2, 3, 4, 0, 1, 2]))
        ),
        test_undirected_graph_with_cycle: (
            vec![
                vec![false, true, false, false, true],
                vec![true, false, true, false, false],
                vec![false, true, false, true, false],
                vec![false, false, true, false, true],
                vec![true, false, false, true, false],
            ],
            2,
            Ok(Some(vec![2, 1, 0, 4, 3, 2]))
        ),
        test_directed_graph_no_cycle: (
            vec![
                vec![false, true, false, true, false],
                vec![false, false, true, true, false],
                vec![false, false, false, true, false],
                vec![false, false, false, false, true],
                vec![false, false, true, false, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_undirected_graph_no_cycle: (
            vec![
                vec![false, true, false, false, false],
                vec![true, false, true, true, false],
                vec![false, true, false, true, true],
                vec![false, true, true, false, true],
                vec![false, false, true, true, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_triangle_graph: (
            vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, false, false],
            ],
            1,
            Ok(Some(vec![1, 2, 0, 1]))
        ),
        test_tree_graph: (
            vec![
                vec![false, true, false, true, false],
                vec![true, false, true, true, false],
                vec![false, true, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, false, true, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_empty_graph: (
            vec![],
            0,
            Err(FindHamiltonianCycleError::EmptyAdjacencyMatrix)
        ),
        test_improper_graph: (
            vec![
                vec![false, true],
                vec![true],
                vec![false, true, true],
                vec![true, true, true, false]
            ],
            0,
            Err(FindHamiltonianCycleError::ImproperAdjacencyMatrix)
        ),
        test_start_out_of_bound: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            3,
            Err(FindHamiltonianCycleError::StartOutOfBound)
        ),
        test_complex_directed_graph: (
            vec![
                vec![false, true, false, true, false, false],
                vec![false, false, true, false, true, false],
                vec![false, false, false, true, false, false],
                vec![false, true, false, false, true, false],
                vec![false, false, true, false, false, true],
                vec![true, false, false, false, false, false],
            ],
            0,
            Ok(Some(vec![0, 1, 2, 3, 4, 5, 0]))
        ),
        single_node_self_loop: (
            vec![
                vec![true],
            ],
            0,
            Ok(Some(vec![0, 0]))
        ),
        single_node: (
            vec![
                vec![false],
            ],
            0,
            Ok(None)
        ),
    }
}
