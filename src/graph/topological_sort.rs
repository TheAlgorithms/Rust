use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq)]
pub enum TopoligicalSortError {
    CycleDetected,
}

type TopologicalSortResult<Node> = Result<Vec<Node>, TopoligicalSortError>;

/// Given a directed graph, modeled as a list of edges from source to destination
/// Uses Kahn's algorithm to either:
///     return the topological sort of the graph
///     or detect if there's any cycle
pub fn topological_sort<Node: Hash + Eq + Copy>(
    edges: &Vec<(Node, Node)>,
) -> TopologicalSortResult<Node> {
    // Preparation:
    //  Build a map of edges, organised from source to destinations
    //  Also, count the number of incoming edges by node
    let mut edges_by_source: HashMap<Node, Vec<Node>> = HashMap::default();
    let mut incoming_edges_count: HashMap<Node, usize> = HashMap::default();
    for (source, destination) in edges {
        incoming_edges_count.entry(*source).or_insert(0); // if we haven't seen this node yet, mark it as having 0 incoming nodes
        edges_by_source // add destination to the list of outgoing edges from source
            .entry(*source)
            .or_default()
            .push(*destination);
        // then make destination have one more incoming edge
        *incoming_edges_count.entry(*destination).or_insert(0) += 1;
    }

    // Now Kahn's algorithm:
    // Add nodes that have no incoming edges to a queue
    let mut no_incoming_edges_q = VecDeque::default();
    for (node, count) in &incoming_edges_count {
        if *count == 0 {
            no_incoming_edges_q.push_back(*node);
        }
    }
    // For each node in this "O-incoming-edge-queue"
    let mut sorted = Vec::default();
    while let Some(no_incoming_edges) = no_incoming_edges_q.pop_back() {
        sorted.push(no_incoming_edges); // since the node has no dependency, it can be safely pushed to the sorted result
        incoming_edges_count.remove(&no_incoming_edges);
        // For each node having this one as dependency
        for neighbour in edges_by_source.get(&no_incoming_edges).unwrap_or(&vec![]) {
            if let Some(count) = incoming_edges_count.get_mut(neighbour) {
                *count -= 1; // decrement the count of incoming edges for the dependent node
                if *count == 0 {
                    // `node` was the last node `neighbour` was dependent on
                    incoming_edges_count.remove(neighbour); // let's remove it from the map, so that we can know if we covered the whole graph
                    no_incoming_edges_q.push_front(*neighbour); // it has no incoming edges anymore => push it to the queue
                }
            }
        }
    }
    if incoming_edges_count.is_empty() {
        // we have visited every node
        Ok(sorted)
    } else {
        // some nodes haven't been visited, meaning there's a cycle in the graph
        Err(TopoligicalSortError::CycleDetected)
    }
}

#[cfg(test)]
mod tests {
    use super::topological_sort;
    use crate::graph::topological_sort::TopoligicalSortError;

    fn is_valid_sort<Node: Eq>(sorted: &[Node], graph: &[(Node, Node)]) -> bool {
        for (source, dest) in graph {
            let source_pos = sorted.iter().position(|node| node == source);
            let dest_pos = sorted.iter().position(|node| node == dest);
            match (source_pos, dest_pos) {
                (Some(src), Some(dst)) if src < dst => {}
                _ => {
                    return false;
                }
            };
        }
        true
    }

    #[test]
    fn it_works() {
        let graph = vec![(1, 2), (1, 3), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)];
        let sort = topological_sort(&graph);
        assert!(sort.is_ok());
        let sort = sort.unwrap();
        assert!(is_valid_sort(&sort, &graph));
        assert_eq!(sort, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_wikipedia_example() {
        let graph = vec![
            (5, 11),
            (7, 11),
            (7, 8),
            (3, 8),
            (3, 10),
            (11, 2),
            (11, 9),
            (11, 10),
            (8, 9),
        ];
        let sort = topological_sort(&graph);
        assert!(sort.is_ok());
        let sort = sort.unwrap();
        assert!(is_valid_sort(&sort, &graph));
    }

    #[test]
    fn test_cyclic_graph() {
        let graph = vec![(1, 2), (2, 3), (3, 4), (4, 5), (4, 2)];
        let sort = topological_sort(&graph);
        assert!(sort.is_err());
        assert_eq!(sort.err().unwrap(), TopoligicalSortError::CycleDetected);
    }
}
