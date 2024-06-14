//! The Ford-Fulkerson algorithm is a widely used algorithm to solve the maximum flow problem in a flow network.
//!
//! The maximum flow problem involves determining the maximum amount of flow that can be sent from a source vertex to a sink vertex
//! in a directed weighted graph, subject to capacity constraints on the edges.
//!
//! The following is the simple idea of the Ford-Fulkerson algorithm:
//!
//! 1. Start with the initial flow as 0.
//! 2. While there exists an augmenting path from the source to the sink:
//!     - Find an augmenting path using any path-finding algorithm, such as breadth-first search or depth-first search.
//!     - Determine the amount of flow that can be sent along the augmenting path, which is the minimum residual capacity along the edges of the path.
//!     - Increase the flow along the augmenting path by the determined amount.
//! 3. Return the maximum flow and residual graph.

use std::collections::VecDeque;

/// Performs a Breadth-First Search (BFS) on the residual graph to find an augmenting path
/// from the source vertex `source` to the sink vertex `sink`.
///
/// # Arguments
///
/// * `graph` - A mutable reference to the residual graph represented as an adjacency matrix.
/// * `source` - The source vertex.
/// * `sink` - The sink vertex.
/// * `parent` - A mutable reference to the parent array used to store the augmenting path.
///
/// # Returns
///
/// Returns `true` if an augmenting path is found from `source` to `sink`, `false` otherwise.
fn bfs(graph: &[Vec<i32>], source: usize, sink: usize, parent: &mut [i32]) -> bool {
    let mut visited = vec![false; graph.len()];
    visited[source] = true;
    parent[source] = -1;

    let mut queue = VecDeque::new();
    queue.push_back(source);

    while let Some(current_vertex) = queue.pop_front() {
        for (previous_vertex, &capacity) in graph[current_vertex].iter().enumerate() {
            if !visited[previous_vertex] && capacity > 0 {
                visited[previous_vertex] = true;
                parent[previous_vertex] = current_vertex as i32;
                if previous_vertex == sink {
                    return true;
                }
                queue.push_back(previous_vertex);
            }
        }
    }

    false
}

/// Applies the Ford-Fulkerson algorithm to find the maximum flow in a flow network
/// represented by a weighted directed graph.
///
/// # Arguments
///
/// * `graph` - A mutable reference to the flow network represented as an adjacency matrix.
/// * `source` - The source vertex.
/// * `sink` - The sink vertex.
///
/// # Returns
///
/// Returns the maximum flow and the residual graph
pub fn ford_fulkerson(graph: &[Vec<i32>], source: usize, sink: usize) -> (i32, Vec<Vec<i32>>) {
    let mut residual_graph = graph.to_owned();
    let mut parent = vec![-1; graph.len()];
    let mut max_flow = 0;

    while bfs(&residual_graph, source, sink, &mut parent) {
        let mut path_flow = i32::MAX;
        let mut previous_vertex = sink;

        while previous_vertex != source {
            let current_vertex = parent[previous_vertex] as usize;
            path_flow = path_flow.min(residual_graph[current_vertex][previous_vertex]);
            previous_vertex = current_vertex;
        }

        previous_vertex = sink;
        while previous_vertex != source {
            let current_vertex = parent[previous_vertex] as usize;
            residual_graph[current_vertex][previous_vertex] -= path_flow;
            residual_graph[previous_vertex][current_vertex] += path_flow;
            previous_vertex = current_vertex;
        }

        max_flow += path_flow;
    }

    (max_flow, residual_graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_max_flow {
        ($($name:ident: $tc:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let (graph, source, sink, expected_flow, expected_residual_graph) = $tc;
                    let (max_flow, residual_graph) = ford_fulkerson(&graph, source, sink);
                    assert_eq!(max_flow, expected_flow);
                    assert_eq!(residual_graph, expected_residual_graph);
                }
            )*
        };
    }

    test_max_flow! {
        test_example_1: (
            vec![
                vec![0, 12, 0, 13, 0, 0],
                vec![0, 0, 10, 0, 0, 0],
                vec![0, 0, 0, 13, 3, 15],
                vec![0, 0, 7, 0, 15, 0],
                vec![0, 0, 6, 0, 0, 17],
                vec![0, 0, 0, 0, 0, 0],
            ],
            0,
            5,
            23,
            vec![
                vec![0, 2, 0, 0, 0, 0],
                vec![10, 0, 0, 0, 0, 0],
                vec![0, 10, 0, 18, 3, 0],
                vec![13, 0, 2, 0, 7, 0],
                vec![0, 0, 6, 8, 0, 9],
                vec![0, 0, 15, 0, 8, 0],
            ],
        ),
        test_example_2: (
            vec![
                vec![0, 4, 0, 3, 0, 0],
                vec![0, 0, 4, 0, 8, 0],
                vec![0, 0, 0, 3, 0, 2],
                vec![0, 0, 0, 0, 6, 0],
                vec![0, 0, 6, 0, 0, 6],
                vec![0, 0, 0, 0, 0, 0],
            ],
            0,
            5,
            7,
            vec![
                vec![0, 0, 0, 0, 0, 0],
                vec![4, 0, 2, 0, 6, 0],
                vec![0, 2, 0, 3, 0, 0],
                vec![3, 0, 0, 0, 3, 0],
                vec![0, 2, 6, 3, 0, 1],
                vec![0, 0, 2, 0, 5, 0],
            ],
        ),
        test_example_3: (
            vec![
                vec![0, 10, 0, 10, 0, 0],
                vec![0, 0, 4, 2, 8, 0],
                vec![0, 0, 0, 0, 0, 10],
                vec![0, 0, 0, 0, 9, 0],
                vec![0, 0, 6, 0, 0, 10],
                vec![0, 0, 0, 0, 0, 0],
            ],
            0,
            5,
            19,
            vec![
                vec![0, 0, 0, 1, 0, 0],
                vec![10, 0, 0, 2, 2, 0],
                vec![0, 4, 0, 0, 5, 1],
                vec![9, 0, 0, 0, 0, 0],
                vec![0, 6, 1, 9, 0, 0],
                vec![0, 0, 9, 0, 10, 0],
            ],
        ),
        test_example_4: (
            vec![
                vec![0, 8, 0, 0, 3, 0],
                vec![0, 0, 9, 0, 0, 0],
                vec![0, 0, 0, 0, 7, 2],
                vec![0, 0, 0, 0, 0, 5],
                vec![0, 0, 7, 4, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
            ],
            0,
            5,
            6,
            vec![
                vec![0, 5, 0, 0, 0, 0],
                vec![3, 0, 6, 0, 0, 0],
                vec![0, 3, 0, 0, 6, 0],
                vec![0, 0, 0, 0, 4, 1],
                vec![3, 0, 8, 0, 0, 0],
                vec![0, 0, 2, 4, 0, 0],
            ],
        ),
        test_example_5: (
            vec![
                vec![0, 16, 13, 0, 0, 0],
                vec![0, 0, 10, 12, 0, 0],
                vec![0, 4, 0, 0, 14, 0],
                vec![0, 0, 9, 0, 0, 20],
                vec![0, 0, 0, 7, 0, 4],
                vec![0, 0, 0, 0, 0, 0],
            ],
            0,
            5,
            23,
            vec![
                vec![0, 4, 2, 0, 0, 0],
                vec![12, 0, 10, 0, 0, 0],
                vec![11, 4, 0, 0, 3, 0],
                vec![0, 12, 9, 0, 7, 1],
                vec![0, 0, 11, 0, 0, 0],
                vec![0, 0, 0, 19, 4, 0],
            ],
        ),
        test_example_6: (
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
            ],
            0,
            3,
            0,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
            ],
        ),
    }
}