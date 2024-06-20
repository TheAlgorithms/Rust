//! This module provides functionality to find a Hamiltonian cycle in a graph.
//! Source: [Wikipedia](https://en.wikipedia.org/wiki/Hamiltonian_path_problem)

/// Represents potential errors when working with an adjacency matrix.
#[derive(Debug, PartialEq, Eq)]
pub enum AdjMatError {
    /// Indicates that the adjacency matrix is empty.
    EmptyMat,
    /// Indicates that the adjacency matrix is not square.
    ImproperMat,
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
    /// A `Result` containing the graph if successful, or an `AdjMatError` if there is an issue with the matrix.
    fn new(adjacency_matrix: Vec<Vec<bool>>) -> Result<Self, AdjMatError> {
        // Check if the adjacency matrix is empty.
        if adjacency_matrix.is_empty() {
            return Err(AdjMatError::EmptyMat);
        }

        // Validate that the adjacency matrix is square.
        if adjacency_matrix
            .iter()
            .any(|row| row.len() != adjacency_matrix.len())
        {
            return Err(AdjMatError::ImproperMat);
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
    /// * `path` - A reference to the current path being explored.
    /// * `pos` - The position of the current vertex being considered.
    ///
    /// # Returns
    ///
    /// `true` if it is safe to include `v` in the path, `false` otherwise.
    fn is_safe(&self, v: usize, path: &[usize], pos: usize) -> bool {
        // Check if the current vertex and the last vertex in the path are adjacent.
        if !self.adjacency_matrix[path[pos - 1]][v] {
            return false;
        }

        // Check if the vertex has already been included in the path.
        !path[..pos].contains(&v)
    }

    /// Recursively searches for a Hamiltonian cycle.
    ///
    /// This function is called by `find_hamiltonian_cycle`.
    ///
    /// # Arguments
    ///
    /// * `path` - A mutable vector representing the current path being explored.
    /// * `pos` - The position of the current vertex being considered.
    ///
    /// # Returns
    ///
    /// `true` if a Hamiltonian cycle is found, `false` otherwise.
    fn hamiltonian_cycle_util(&self, path: &mut Vec<usize>, pos: usize) -> bool {
        if pos == self.num_vertices() {
            // Check if there is an edge from the last included vertex to the first vertex.
            return self.adjacency_matrix[path[pos - 1]][path[0]];
        }

        for v in 0..self.num_vertices() {
            if self.is_safe(v, path, pos) {
                path[pos] = v;
                if self.hamiltonian_cycle_util(path, pos + 1) {
                    return true;
                }
                path[pos] = usize::MAX;
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
    ) -> Result<Option<Vec<usize>>, AdjMatError> {
        // Validate the start vertex.
        if start_vertex >= self.num_vertices() {
            return Err(AdjMatError::StartOutOfBound);
        }

        // Initialize the path.
        let mut path = vec![usize::MAX; self.adjacency_matrix.len()];
        // Start at the specified vertex.
        path[0] = start_vertex;

        if self.hamiltonian_cycle_util(&mut path, 1) {
            // Complete the cycle by returning to the starting vertex.
            path.push(start_vertex);
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}

/// Attempts to find a Hamiltonian cycle in a graph represented by an adjacency matrix, starting from a specified vertex.
pub fn find_hamiltonian_cycle(
    adjacency_matrix: Vec<Vec<bool>>,
    start_vertex: usize,
) -> Result<Option<Vec<usize>>, AdjMatError> {
    let graph = Graph::new(adjacency_matrix)?;
    graph.find_hamiltonian_cycle(start_vertex)
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
        test_cycle_graph: (
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
        test_no_cycle_graph: (
            vec![
                vec![false, true, false],
                vec![true, false, false],
                vec![false, false, false],
            ],
            0,
            Ok(None::<Vec<usize>>)
        ),
        test_triangle_graph: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            1,
            Ok(Some(vec![1, 0, 2, 1]))
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
        test_empty_matrix: (
            vec![],
            0,
            Err(AdjMatError::EmptyMat)
        ),
        test_improper_matrix: (
            vec![
                vec![false, true],
                vec![true],
                vec![false, true, true],
                vec![true, true, true, false]
            ],
            0,
            Err(AdjMatError::ImproperMat)
        ),
        test_start_out_of_bound: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            3,
            Err(AdjMatError::StartOutOfBound)
        ),
    }
}
