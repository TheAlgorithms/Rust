//! This module provides functionality for generating all possible colorings of a undirected (or directed) graph
//! given a certain number of colors. It includes the GraphColoring struct and methods
//! for validating color assignments and finding all valid colorings.

/// Represents potential errors when coloring on an adjacency matrix.
#[derive(Debug, PartialEq, Eq)]
pub enum GraphColoringError {
    // Indicates that the adjacency matrix is empty
    EmptyAdjacencyMatrix,
    // Indicates that the adjacency matrix is not squared
    ImproperAdjacencyMatrix,
}

/// Generates all possible valid colorings of a graph.
///
/// # Arguments
///
/// * `adjacency_matrix` - A 2D vector representing the adjacency matrix of the graph.
/// * `num_colors` - The number of colors available for coloring the graph.
///
/// # Returns
///
/// * A `Result` containing an `Option` with a vector of solutions or a `GraphColoringError` if
/// there is an issue with the matrix.
pub fn generate_colorings(
    adjacency_matrix: Vec<Vec<bool>>,
    num_colors: usize,
) -> Result<Option<Vec<Vec<usize>>>, GraphColoringError> {
    Ok(GraphColoring::new(adjacency_matrix)?.find_solutions(num_colors))
}

/// A struct representing a graph coloring problem.
struct GraphColoring {
    // The adjacency matrix of the graph
    adjacency_matrix: Vec<Vec<bool>>,
    // The current colors assigned to each vertex
    vertex_colors: Vec<usize>,
    // Vector of all valid color assignments for the vertices found during the search
    solutions: Vec<Vec<usize>>,
}

impl GraphColoring {
    /// Creates a new GraphColoring instance.
    ///
    /// # Arguments
    ///
    /// * `adjacency_matrix` - A 2D vector representing the adjacency matrix of the graph.
    ///
    /// # Returns
    ///
    /// * A new instance of GraphColoring or a `GraphColoringError` if the matrix is empty or non-square.
    fn new(adjacency_matrix: Vec<Vec<bool>>) -> Result<Self, GraphColoringError> {
        let num_vertices = adjacency_matrix.len();

        // Check if the adjacency matrix is empty
        if num_vertices == 0 {
            return Err(GraphColoringError::EmptyAdjacencyMatrix);
        }

        // Check if the adjacency matrix is square
        if adjacency_matrix.iter().any(|row| row.len() != num_vertices) {
            return Err(GraphColoringError::ImproperAdjacencyMatrix);
        }

        Ok(GraphColoring {
            adjacency_matrix,
            vertex_colors: vec![usize::MAX; num_vertices],
            solutions: Vec::new(),
        })
    }

    /// Returns the number of vertices in the graph.
    fn num_vertices(&self) -> usize {
        self.adjacency_matrix.len()
    }

    /// Checks if a given color can be assigned to a vertex without causing a conflict.
    ///
    /// # Arguments
    ///
    /// * `vertex` - The index of the vertex to be colored.
    /// * `color` - The color to be assigned to the vertex.
    ///
    /// # Returns
    ///
    /// * `true` if the color can be assigned to the vertex, `false` otherwise.
    fn is_color_valid(&self, vertex: usize, color: usize) -> bool {
        for neighbor in 0..self.num_vertices() {
            // Check outgoing edges from vertex and incoming edges to vertex
            if (self.adjacency_matrix[vertex][neighbor] || self.adjacency_matrix[neighbor][vertex])
                && self.vertex_colors[neighbor] == color
            {
                return false;
            }
        }
        true
    }

    /// Recursively finds all valid colorings for the graph.
    ///
    /// # Arguments
    ///
    /// * `vertex` - The current vertex to be colored.
    /// * `num_colors` - The number of colors available for coloring the graph.
    fn find_colorings(&mut self, vertex: usize, num_colors: usize) {
        if vertex == self.num_vertices() {
            self.solutions.push(self.vertex_colors.clone());
            return;
        }

        for color in 0..num_colors {
            if self.is_color_valid(vertex, color) {
                self.vertex_colors[vertex] = color;
                self.find_colorings(vertex + 1, num_colors);
                self.vertex_colors[vertex] = usize::MAX;
            }
        }
    }

    /// Finds all solutions for the graph coloring problem.
    ///
    /// # Arguments
    ///
    /// * `num_colors` - The number of colors available for coloring the graph.
    ///
    /// # Returns
    ///
    /// * A `Result` containing an `Option` with a vector of solutions or a `GraphColoringError`.
    fn find_solutions(&mut self, num_colors: usize) -> Option<Vec<Vec<usize>>> {
        self.find_colorings(0, num_colors);
        if self.solutions.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.solutions))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_graph_coloring {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (adjacency_matrix, num_colors, expected) = $test_case;
                    let actual = generate_colorings(adjacency_matrix, num_colors);
                    assert_eq!(actual, expected);
                }
            )*
        };
    }

    test_graph_coloring! {
        test_complete_graph_with_3_colors: (
            vec![
                vec![false, true, true, true],
                vec![true, false, true, false],
                vec![true, true, false, true],
                vec![true, false, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1],
                vec![0, 2, 1, 2],
                vec![1, 0, 2, 0],
                vec![1, 2, 0, 2],
                vec![2, 0, 1, 0],
                vec![2, 1, 0, 1],
            ]))
        ),
        test_linear_graph_with_2_colors: (
            vec![
                vec![false, true, false, false],
                vec![true, false, true, false],
                vec![false, true, false, true],
                vec![false, false, true, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 1, 0, 1],
                vec![1, 0, 1, 0],
            ]))
        ),
        test_incomplete_graph_with_insufficient_colors: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            1,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_empty_graph: (
            vec![],
            1,
            Err(GraphColoringError::EmptyAdjacencyMatrix)
        ),
        test_non_square_matrix: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
            ],
            3,
            Err(GraphColoringError::ImproperAdjacencyMatrix)
        ),
        test_single_vertex_graph: (
            vec![
                vec![false],
            ],
            1,
            Ok(Some(vec![
                vec![0],
            ]))
        ),
        test_bipartite_graph_with_2_colors: (
            vec![
                vec![false, true, false, true],
                vec![true, false, true, false],
                vec![false, true, false, true],
                vec![true, false, true, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 1, 0, 1],
                vec![1, 0, 1, 0],
            ]))
        ),
        test_large_graph_with_3_colors: (
            vec![
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
                vec![true, false, true, true, false, true, true, false, true, true],
                vec![true, true, false, true, true, false, true, true, false, true],
                vec![false, true, true, false, true, true, false, true, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 0, 1, 2, 0, 1, 2, 0],
                vec![0, 2, 1, 0, 2, 1, 0, 2, 1, 0],
                vec![1, 0, 2, 1, 0, 2, 1, 0, 2, 1],
                vec![1, 2, 0, 1, 2, 0, 1, 2, 0, 1],
                vec![2, 0, 1, 2, 0, 1, 2, 0, 1, 2],
                vec![2, 1, 0, 2, 1, 0, 2, 1, 0, 2],
            ]))
        ),
        test_disconnected_graph: (
            vec![
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
            ],
            2,
            Ok(Some(vec![
                vec![0, 0, 0],
                vec![0, 0, 1],
                vec![0, 1, 0],
                vec![0, 1, 1],
                vec![1, 0, 0],
                vec![1, 0, 1],
                vec![1, 1, 0],
                vec![1, 1, 1],
            ]))
        ),
        test_no_valid_coloring: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            2,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_more_colors_than_nodes: (
            vec![
                vec![true, true],
                vec![true, true],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
            ]))
        ),
        test_no_coloring_with_zero_colors: (
            vec![
                vec![true],
            ],
            0,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_complete_graph_with_3_vertices_and_3_colors: (
            vec![
                vec![false, true, true],
                vec![true, false, true],
                vec![true, true, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
            ]))
        ),
        test_directed_graph_with_3_colors: (
            vec![
                vec![false, true, false, true],
                vec![false, false, true, false],
                vec![true, false, false, true],
                vec![true, false, false, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1],
                vec![0, 2, 1, 2],
                vec![1, 0, 2, 0],
                vec![1, 2, 0, 2],
                vec![2, 0, 1, 0],
                vec![2, 1, 0, 1],
            ]))
        ),
        test_directed_graph_no_valid_coloring: (
            vec![
                vec![false, true, false, true],
                vec![false, false, true, true],
                vec![true, false, false, true],
                vec![true, false, false, false],
            ],
            3,
            Ok(None::<Vec<Vec<usize>>>)
        ),
        test_large_directed_graph_with_3_colors: (
            vec![
                vec![false, true, false, false, true, false, false, true, false, false],
                vec![false, false, true, false, false, true, false, false, true, false],
                vec![false, false, false, true, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
                vec![false, true, false, false, false, true, false, false, true, false],
                vec![false, false, true, false, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
                vec![false, true, false, false, false, true, false, false, true, false],
                vec![false, false, true, false, false, false, true, false, false, true],
                vec![true, false, false, false, true, false, false, true, false, false],
            ],
            3,
            Ok(Some(vec![
                vec![0, 1, 2, 1, 2, 0, 1, 2, 0, 1],
                vec![0, 2, 1, 2, 1, 0, 2, 1, 0, 2],
                vec![1, 0, 2, 0, 2, 1, 0, 2, 1, 0],
                vec![1, 2, 0, 2, 0, 1, 2, 0, 1, 2],
                vec![2, 0, 1, 0, 1, 2, 0, 1, 2, 0],
                vec![2, 1, 0, 1, 0, 2, 1, 0, 2, 1]
            ]))
        ),
    }
}
