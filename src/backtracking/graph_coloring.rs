//! This module provides functionality for generating all possible colorings of a graph
//! given a certain number of colors. It includes the GraphColoring struct and methods
//! for validating color assignments and finding all valid colorings.

/// Generates all possible valid colorings of a graph.
///
/// # Arguments
///
/// * `adj_matrix` - A 2D vector representing the adjacency matrix of the graph.
/// * `num_colors` - The number of colors available for coloring the graph.
///
/// # Returns
///
/// * An `Option` containing a vector of solutions. Each solution is a vector of color assignments for the vertices.
pub fn generate_colorings(
    adj_matrix: Vec<Vec<usize>>,
    num_colors: usize,
) -> Option<Vec<Vec<usize>>> {
    let mut graph_coloring = GraphColoring::new(adj_matrix, num_colors);
    graph_coloring.find_solutions()
}

/// A struct representing a graph coloring problem.
struct GraphColoring {
    adj_matrix: Vec<Vec<usize>>,
    num_colors: usize,
    vertex_colors: Vec<usize>,
    solutions: Vec<Vec<usize>>,
}

impl GraphColoring {
    /// Creates a new GraphColoring instance.
    ///
    /// # Arguments
    ///
    /// * `adj_matrix` - A 2D vector representing the adjacency matrix of the graph.
    /// * `num_colors` - The number of colors available for coloring the graph.
    ///
    /// # Returns
    ///
    /// * A new instance of GraphColoring.
    fn new(adj_matrix: Vec<Vec<usize>>, num_colors: usize) -> Self {
        let num_vertices = adj_matrix.len();
        GraphColoring {
            adj_matrix,
            num_colors,
            vertex_colors: vec![0; num_vertices],
            solutions: Vec::new(),
        }
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
        for neighbor in 0..self.adj_matrix.len() {
            if self.adj_matrix[vertex][neighbor] == 1 && self.vertex_colors[neighbor] == color {
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
    fn find_colorings(&mut self, vertex: usize) {
        if vertex == self.adj_matrix.len() {
            self.solutions.push(self.vertex_colors.clone());
            return;
        }

        for color in 1..=self.num_colors {
            if self.is_color_valid(vertex, color) {
                self.vertex_colors[vertex] = color;
                self.find_colorings(vertex + 1);
                self.vertex_colors[vertex] = 0;
            }
        }
    }

    /// Finds all solutions for the graph coloring problem.
    ///
    /// # Returns
    ///
    /// * An `Option` containing a vector of solutions. Each solution is a vector of color assignments for the vertices.
    fn find_solutions(&mut self) -> Option<Vec<Vec<usize>>> {
        self.find_colorings(0);
        if self.solutions.is_empty() {
            None
        } else {
            Some(self.solutions.clone())
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
                    let (adj_matrix, num_colors, expected) = $test_case;
                    let actual = generate_colorings(adj_matrix, num_colors);
                    assert_eq!(actual, expected);
                }
            )*
        };
    }

    test_graph_coloring! {
        test_complete_graph_with_3_colors: (
            vec![
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0],
                vec![1, 1, 0, 1],
                vec![1, 0, 1, 0],
            ],
            3,
            Some(vec![
                vec![1, 2, 3, 2],
                vec![1, 3, 2, 3],
                vec![2, 1, 3, 1],
                vec![2, 3, 1, 3],
                vec![3, 1, 2, 1],
                vec![3, 2, 1, 2],
            ])
        ),
        test_linear_graph_with_2_colors: (
            vec![
                vec![0, 1, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 0, 1],
                vec![0, 0, 1, 0],
            ],
            2,
            Some(vec![
                vec![1, 2, 1, 2],
                vec![2, 1, 2, 1],
            ])
        ),
        test_incomplete_graph_with_insufficient_colors: (
            vec![
                vec![0, 1, 1],
                vec![1, 0, 1],
                vec![0, 1, 0],
            ],
            1,
            None::<Vec<Vec<usize>>>
        ),
        test_empty_graph: (
            vec![
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
            ],
            1,
            Some(vec![
                vec![1, 1, 1],
            ])
        ),
        test_single_vertex_graph: (
            vec![
                vec![0],
            ],
            1,
            Some(vec![
                vec![1],
            ])
        ),
        test_bipartite_graph_with_2_colors: (
            vec![
                vec![0, 1, 0, 1],
                vec![1, 0, 1, 0],
                vec![0, 1, 0, 1],
                vec![1, 0, 1, 0],
            ],
            2,
            Some(vec![
                vec![1, 2, 1, 2],
                vec![2, 1, 2, 1],
            ])
        ),
        test_large_graph_with_3_colors: (
            vec![
                vec![0, 1, 1, 0, 1, 1, 0, 1, 1, 0],
                vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1],
                vec![1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
                vec![0, 1, 1, 0, 1, 1, 0, 1, 1, 0],
                vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1],
                vec![1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
                vec![0, 1, 1, 0, 1, 1, 0, 1, 1, 0],
                vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1],
                vec![1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
                vec![0, 1, 1, 0, 1, 1, 0, 1, 1, 0],
            ],
            3,
            Some(vec![
                vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1],
                vec![1, 3, 2, 1, 3, 2, 1, 3, 2, 1],
                vec![2, 1, 3, 2, 1, 3, 2, 1, 3, 2],
                vec![2, 3, 1, 2, 3, 1, 2, 3, 1, 2],
                vec![3, 1, 2, 3, 1, 2, 3, 1, 2, 3],
                vec![3, 2, 1, 3, 2, 1, 3, 2, 1, 3],
            ])
        ),
    }
}
