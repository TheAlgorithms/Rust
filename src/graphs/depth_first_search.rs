use crate::data_structures::SearchableGraph;

pub fn depth_first_search(
    graph: &impl SearchableGraph,
    source: usize,
    target: usize,
) -> Option<Vec<usize>> {
    if source == target {
        return Some(vec![source]);
    }

    // Create data structures
    let mut search_stack = Vec::new();
    let mut discovered = vec![None; graph.num_nodes()];

    // Initialise starting node
    search_stack.push(source);
    discovered[source] = Some(source);

    while let Some(cur_node) = search_stack.pop() {
        for neighbour in graph.neighbours(cur_node) {
            // If not already discovered, add to search stack
            if discovered[neighbour].is_some() {
                continue;
            }
            search_stack.push(neighbour);
            discovered[neighbour] = Some(cur_node);

            // Check if found destination
            if neighbour == target {
                return Some(convert_to_path(discovered, target));
            }
        }
    }
    None
}

// Converts a list of discovered nodes into a path from source to destination
fn convert_to_path(discovered: Vec<Option<usize>>, mut target: usize) -> Vec<usize> {
    let mut path = vec![target];

    // Traverse list from target to source through parent links
    while discovered[target] != Some(target) {
        target = discovered[target].unwrap();
        path.push(target);
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::MatrixGraph;

    #[test]
    fn single_node() {
        let graph = MatrixGraph::new(vec![vec![0]]);
        assert_eq!(Some(vec![0]), depth_first_search(&graph, 0, 0));
    }

    #[test]
    fn simple_path() {
        let graph = simple_graph();
        assert_eq!(Some(vec![0, 2, 1, 3]), depth_first_search(&graph, 0, 3));
    }

    fn simple_graph() -> MatrixGraph<usize> {
        let edges = vec![
            vec![0, 0, 1, 0],
            vec![1, 1, 1, 1],
            vec![0, 1, 0, 0],
            vec![1, 0, 0, 0],
        ];

        MatrixGraph::new(edges)
    }

    // A breadth-first algorithm would find the wrong path
    #[test]
    fn depth_first_matters() {
        let edges = vec![
            vec![0, 1, 0, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 1],
            vec![0, 1, 0, 1, 0, 1],
            vec![0, 0, 0, 0, 1, 0],
            vec![1, 0, 1, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 0],
        ];
        let graph = MatrixGraph::new(edges);

        assert_eq!(Some(vec![2, 5, 0, 4]), depth_first_search(&graph, 2, 4));
    }
}
