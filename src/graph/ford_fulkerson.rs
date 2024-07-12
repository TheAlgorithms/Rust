//! The Ford-Fulkerson algorithm is a widely used algorithm to solve the maximum flow problem in a flow network.
//!
//! The maximum flow problem involves determining the maximum amount of flow that can be sent from a source vertex to a sink vertex
//! in a directed weighted graph, subject to capacity constraints on the edges.

use std::collections::VecDeque;

/// Enum representing the possible errors that can occur when running the Ford-Fulkerson algorithm.
#[derive(Debug, PartialEq)]
pub enum FordFulkersonError {
    EmptyGraph,
    ImproperGraph,
    SourceOutOfBounds,
    SinkOutOfBounds,
}

/// Performs a Breadth-First Search (BFS) on the residual graph to find an augmenting path
/// from the source vertex `source` to the sink vertex `sink`.
///
/// # Arguments
///
/// * `graph` - A reference to the residual graph represented as an adjacency matrix.
/// * `source` - The source vertex.
/// * `sink` - The sink vertex.
/// * `parent` - A mutable reference to the parent array used to store the augmenting path.
///
/// # Returns
///
/// Returns `true` if an augmenting path is found from `source` to `sink`, `false` otherwise.
fn bfs(graph: &[Vec<usize>], source: usize, sink: usize, parent: &mut [usize]) -> bool {
    let mut visited = vec![false; graph.len()];
    visited[source] = true;
    parent[source] = usize::MAX;

    let mut queue = VecDeque::new();
    queue.push_back(source);

    while let Some(current_vertex) = queue.pop_front() {
        for (previous_vertex, &capacity) in graph[current_vertex].iter().enumerate() {
            if !visited[previous_vertex] && capacity > 0 {
                visited[previous_vertex] = true;
                parent[previous_vertex] = current_vertex;
                if previous_vertex == sink {
                    return true;
                }
                queue.push_back(previous_vertex);
            }
        }
    }

    false
}

/// Validates the input parameters for the Ford-Fulkerson algorithm.
///
/// This function checks if the provided graph, source vertex, and sink vertex
/// meet the requirements for the Ford-Fulkerson algorithm. It ensures the graph
/// is non-empty, square (each row has the same length as the number of rows), and
/// that the source and sink vertices are within the valid range of vertex indices.
///
/// # Arguments
///
/// * `graph` - A reference to the flow network represented as an adjacency matrix.
/// * `source` - The source vertex.
/// * `sink` - The sink vertex.
///
/// # Returns
///
/// Returns `Ok(())` if the input parameters are valid, otherwise returns an appropriate
/// `FordFulkersonError`.
fn validate_ford_fulkerson_input(
    graph: &[Vec<usize>],
    source: usize,
    sink: usize,
) -> Result<(), FordFulkersonError> {
    if graph.is_empty() {
        return Err(FordFulkersonError::EmptyGraph);
    }

    if graph.iter().any(|row| row.len() != graph.len()) {
        return Err(FordFulkersonError::ImproperGraph);
    }

    if source >= graph.len() {
        return Err(FordFulkersonError::SourceOutOfBounds);
    }

    if sink >= graph.len() {
        return Err(FordFulkersonError::SinkOutOfBounds);
    }

    Ok(())
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
pub fn ford_fulkerson(
    graph: &[Vec<usize>],
    source: usize,
    sink: usize,
) -> Result<usize, FordFulkersonError> {
    validate_ford_fulkerson_input(graph, source, sink)?;

    let mut residual_graph = graph.to_owned();
    let mut parent = vec![usize::MAX; graph.len()];
    let mut max_flow = 0;

    while bfs(&residual_graph, source, sink, &mut parent) {
        let mut path_flow = usize::MAX;
        let mut previous_vertex = sink;

        while previous_vertex != source {
            let current_vertex = parent[previous_vertex];
            path_flow = path_flow.min(residual_graph[current_vertex][previous_vertex]);
            previous_vertex = current_vertex;
        }

        previous_vertex = sink;
        while previous_vertex != source {
            let current_vertex = parent[previous_vertex];
            residual_graph[current_vertex][previous_vertex] -= path_flow;
            residual_graph[previous_vertex][current_vertex] += path_flow;
            previous_vertex = current_vertex;
        }

        max_flow += path_flow;
    }

    Ok(max_flow)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_max_flow {
        ($($name:ident: $tc:expr,)* ) => {
            $(
                #[test]
                fn $name() {
                    let (graph, source, sink, expected_result) = $tc;
                    assert_eq!(ford_fulkerson(&graph, source, sink), expected_result);
                }
            )*
        };
    }

    test_max_flow! {
        test_empty_graph: (
            vec![],
            0,
            0,
            Err(FordFulkersonError::EmptyGraph),
        ),
        test_source_out_of_bound: (
            vec![
                vec![0, 8, 0, 0, 3, 0],
                vec![0, 0, 9, 0, 0, 0],
                vec![0, 0, 0, 0, 7, 2],
                vec![0, 0, 0, 0, 0, 5],
                vec![0, 0, 7, 4, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
            ],
            6,
            5,
            Err(FordFulkersonError::SourceOutOfBounds),
        ),
        test_sink_out_of_bound: (
            vec![
                vec![0, 8, 0, 0, 3, 0],
                vec![0, 0, 9, 0, 0, 0],
                vec![0, 0, 0, 0, 7, 2],
                vec![0, 0, 0, 0, 0, 5],
                vec![0, 0, 7, 4, 0, 0],
                vec![0, 0, 0, 0, 0, 0],
            ],
            0,
            6,
            Err(FordFulkersonError::SinkOutOfBounds),
        ),
        test_improper_graph: (
            vec![
                vec![0, 8],
                vec![0],
            ],
            0,
            1,
            Err(FordFulkersonError::ImproperGraph),
        ),
        test_graph_with_small_flow: (
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
            Ok(6),
        ),
        test_graph_with_medium_flow: (
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
            Ok(19),
        ),
        test_graph_with_large_flow: (
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
            Ok(23),
        ),
        test_complex_graph: (
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
            Ok(23),
        ),
        test_disconnected_graph: (
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
            ],
            0,
            3,
            Ok(0),
        ),
        test_unconnected_sink: (
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
            Ok(7),
        ),
        test_no_edges: (
            vec![
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
            ],
            0,
            2,
            Ok(0),
        ),
        test_single_vertex: (
            vec![
                vec![0],
            ],
            0,
            0,
            Ok(0),
        ),
        test_self_loop: (
            vec![
                vec![10, 0],
                vec![0, 0],
            ],
            0,
            1,
            Ok(0),
        ),
        test_same_source_sink: (
            vec![
                vec![0, 10, 10],
                vec![0, 0, 10],
                vec![0, 0, 0],
            ],
            0,
            0,
            Ok(0),
        ),
    }
}
