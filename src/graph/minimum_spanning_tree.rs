//! This module implements Kruskal's algorithm to find the Minimum Spanning Tree (MST)
//! of an undirected, weighted graph using a Disjoint Set Union (DSU) for cycle detection.

use crate::graph::DisjointSetUnion;

/// Represents an edge in the graph with a source, destination, and associated cost.
#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    /// The starting vertex of the edge.
    source: usize,
    /// The ending vertex of the edge.
    destination: usize,
    /// The cost associated with the edge.
    cost: usize,
}

impl Edge {
    /// Creates a new edge with the specified source, destination, and cost.
    pub fn new(source: usize, destination: usize, cost: usize) -> Self {
        Self {
            source,
            destination,
            cost,
        }
    }
}

/// Executes Kruskal's algorithm to compute the Minimum Spanning Tree (MST) of a graph.
///
/// # Parameters
///
/// - `edges`: A vector of `Edge` instances representing all edges in the graph.
/// - `num_vertices`: The total number of vertices in the graph.
///
/// # Returns
///
/// An `Option` containing a tuple with:
///
/// - The total cost of the MST (usize).
/// - A vector of edges that are included in the MST.
///
/// Returns `None` if the graph is disconnected.
///
/// # Complexity
///
/// The time complexity is O(E log E), where E is the number of edges.
pub fn kruskal(mut edges: Vec<Edge>, num_vertices: usize) -> Option<(usize, Vec<Edge>)> {
    let mut dsu = DisjointSetUnion::new(num_vertices);
    let mut mst_cost: usize = 0;
    let mut mst_edges: Vec<Edge> = Vec::with_capacity(num_vertices - 1);

    // Sort edges by cost in ascending order
    edges.sort_unstable_by_key(|edge| edge.cost);

    for edge in edges {
        if mst_edges.len() == num_vertices - 1 {
            break;
        }

        // Attempt to merge the sets containing the edge’s vertices
        if dsu.merge(edge.source, edge.destination) != usize::MAX {
            mst_cost += edge.cost;
            mst_edges.push(edge);
        }
    }

    // Return MST if it includes exactly num_vertices - 1 edges, otherwise None for disconnected graphs
    (mst_edges.len() == num_vertices - 1).then_some((mst_cost, mst_edges))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (edges, num_vertices, expected_result) = $test_case;
                    let actual_result = kruskal(edges, num_vertices);
                    assert_eq!(actual_result, expected_result);
                }
            )*
        };
    }

    test_cases! {
        test_seven_vertices_eleven_edges: (
            vec![
                Edge::new(0, 1, 7),
                Edge::new(0, 3, 5),
                Edge::new(1, 2, 8),
                Edge::new(1, 3, 9),
                Edge::new(1, 4, 7),
                Edge::new(2, 4, 5),
                Edge::new(3, 4, 15),
                Edge::new(3, 5, 6),
                Edge::new(4, 5, 8),
                Edge::new(4, 6, 9),
                Edge::new(5, 6, 11),
            ],
            7,
            Some((39, vec![
                Edge::new(0, 3, 5),
                Edge::new(2, 4, 5),
                Edge::new(3, 5, 6),
                Edge::new(0, 1, 7),
                Edge::new(1, 4, 7),
                Edge::new(4, 6, 9),
            ]))
        ),
        test_ten_vertices_twenty_edges: (
            vec![
                Edge::new(0, 1, 3),
                Edge::new(0, 3, 6),
                Edge::new(0, 4, 9),
                Edge::new(1, 2, 2),
                Edge::new(1, 3, 4),
                Edge::new(1, 4, 9),
                Edge::new(2, 3, 2),
                Edge::new(2, 5, 8),
                Edge::new(2, 6, 9),
                Edge::new(3, 6, 9),
                Edge::new(4, 5, 8),
                Edge::new(4, 9, 18),
                Edge::new(5, 6, 7),
                Edge::new(5, 8, 9),
                Edge::new(5, 9, 10),
                Edge::new(6, 7, 4),
                Edge::new(6, 8, 5),
                Edge::new(7, 8, 1),
                Edge::new(7, 9, 4),
                Edge::new(8, 9, 3),
            ],
            10,
            Some((38, vec![
                Edge::new(7, 8, 1),
                Edge::new(1, 2, 2),
                Edge::new(2, 3, 2),
                Edge::new(0, 1, 3),
                Edge::new(8, 9, 3),
                Edge::new(6, 7, 4),
                Edge::new(5, 6, 7),
                Edge::new(2, 5, 8),
                Edge::new(4, 5, 8),
            ]))
        ),
        test_disconnected_graph: (
            vec![
                Edge::new(0, 1, 4),
                Edge::new(0, 2, 6),
                Edge::new(3, 4, 2),
            ],
            5,
            None
        ),
    }
}
