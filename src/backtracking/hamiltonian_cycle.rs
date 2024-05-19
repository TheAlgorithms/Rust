//! This module provides functionality to find a Hamiltonian cycle in graph.
//! Source: [Wikipedia](https://en.wikipedia.org/wiki/Hamiltonian_path_problem)

/// Represents a graph with an adjacency matrix.
struct Graph {
    /// The adjacency matrix representing the graph.
    adjacency_matrix: Vec<Vec<u8>>,
    /// The number of vertices in the graph.
    vertices: usize,
}

impl Graph {
    /// Creates a new graph with the given adjacency matrix.
    ///
    /// # Arguments
    ///
    /// * `adjacency_matrix` - A square matrix where each element represents
    ///                        the presence `1` or absence `0` of an edge
    /// between two vertices.
    fn new(adjacency_matrix: Vec<Vec<u8>>) -> Self {
        let vertices = adjacency_matrix.len();
        Self {
            adjacency_matrix,
            vertices,
        }
    }

    /// Checks if it's safe to include vertex `v` in the Hamiltonian cycle path.
    ///
    /// # Arguments
    ///
    /// * `v` - The index of the vertex being considered.
    /// * `path` - A reference to the current path being explored.
    /// * `pos` - The position of the current vertex being considered.
    ///
    /// # Returns
    ///
    /// * `true` if it's safe to include `v` in the path, `false` otherwise.
    fn is_safe(&self, v: usize, path: &[usize], pos: usize) -> bool {
        // Check if the current vertex and the last vertex in the path are adjacent
        if self.adjacency_matrix[path[pos - 1]][v] == 0 {
            return false;
        }

        // Check if the vertex has already been included in the path
        !path[..pos].contains(&v)
    }

    /// Utility function for finding a Hamiltonian cycle recursively.
    ///
    /// This function is called recursively by `find_hamiltonian_cycle`.
    ///
    /// # Arguments
    ///
    /// * `path` - A mutable vector representing the current path being explored.
    /// * `pos` - The position of the current vertex being considered.
    ///
    /// # Returns
    ///
    /// * `true` if a Hamiltonian cycle is found, `false` otherwise.
    fn hamiltonian_cycle_util(&self, path: &mut Vec<usize>, pos: usize) -> bool {
        if pos == self.vertices {
            // Check if there is an edge from the last included vertex to the first vertex
            return self.adjacency_matrix[path[pos - 1]][path[0]] == 1;
        }

        for v in 0..self.vertices {
            if self.is_safe(v, path, pos) {
                path[pos] = v;
                if self.hamiltonian_cycle_util(path, pos + 1) {
                    return true;
                }
                path[pos] = std::usize::MAX;
            }
        }

        false
    }

    /// Finds a Hamiltonian cycle in the graph, if one exists, starting from the specified vertex.
    ///
    /// A Hamiltonian cycle is a cycle that visits every vertex exactly once
    /// and returns to the starting vertex.
    ///
    /// Returns `Some(path)` if a Hamiltonian cycle is found, where `path` is a vector
    /// containing the indices of vertices in the cycle, starting and ending with the same vertex.
    ///
    /// Returns `None` if no Hamiltonian cycle exists in the graph.
    fn find_hamiltonian_cycle(&self, start_vertex: usize) -> Option<Vec<usize>> {
        let mut path = vec![std::usize::MAX; self.vertices];
        path[0] = start_vertex; // Start at the specified vertex

        if self.hamiltonian_cycle_util(&mut path, 1) {
            path.push(start_vertex); // To complete the cycle by returning to the starting vertex
            Some(path)
        } else {
            None
        }
    }
}

/// Finds a Hamiltonian cycle in a given graph represented by an adjacency matrix, if one exists, starting from a specified vertex
pub fn find_hamiltonian_cycle(
    adjacency_matrix: Vec<Vec<u8>>,
    start_vertex: usize,
) -> Option<Vec<usize>> {
    let graph = Graph::new(adjacency_matrix);
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
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 1],
                vec![1, 1, 0, 1],
                vec![1, 1, 1, 0],
            ],
            0,
            Some(vec![0, 1, 2, 3, 0])
        ),
        test_cycle_graph: (
            vec![
                vec![0, 1, 0, 0, 1],
                vec![1, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 1, 0, 1],
                vec![1, 0, 0, 1, 0],
            ],
            2,
            Some(vec![2, 1, 0, 4, 3, 2])
        ),
        test_no_cycle_graph: (
            vec![
                vec![0, 1, 0],
                vec![1, 0, 0],
                vec![0, 0, 0],
            ],
            0,
            None::<Vec<usize>>
        ),
        test_triangle_graph: (
            vec![
                vec![0, 1, 1],
                vec![1, 0, 1],
                vec![1, 1, 0],
            ],
            1,
            Some(vec![1, 0, 2, 1])
        ),
        test_tree_graph: (
            vec![
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 1, 1, 0],
                vec![0, 1, 0, 0, 0],
                vec![1, 0, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
            ],
            0,
            None::<Vec<usize>>
        ),
    }
}
