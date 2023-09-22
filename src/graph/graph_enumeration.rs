use std::collections::BTreeMap;

type Graph<Vertex> = BTreeMap<Vertex, Vec<Vertex>>;

/*
This function creates a graph with vertices numbered from 1 to n for any input
`Graph<V>`. The result is in the form of Vec<Vec<usize> to make implementing
other algorithms on the graph easier and help with performance.

We expect that all vertices, even the isolated ones, to have an entry in `adj`
(possibly an empty vector)
*/
pub fn enumerate_graph<V: Ord + Clone>(adj: &Graph<V>) -> Vec<Vec<usize>> {
    let mut result = vec![vec![]; adj.len() + 1];
    let ordering: Vec<V> = adj.keys().cloned().collect();
    for (zero_idx, edges) in adj.values().enumerate() {
        let idx = zero_idx + 1;
        result[idx] = edges
            .iter()
            .map(|x| ordering.binary_search(x).unwrap() + 1)
            .collect();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn add_edge<V: Ord + Clone>(graph: &mut Graph<V>, a: V, b: V) {
        graph.entry(a.clone()).or_default().push(b.clone());
        graph.entry(b).or_default().push(a);
    }

    #[test]
    fn string_vertices() {
        let mut graph = Graph::new();
        add_edge(&mut graph, "a", "b");
        add_edge(&mut graph, "b", "c");
        add_edge(&mut graph, "c", "a");
        add_edge(&mut graph, "b", "d");
        let mut result = enumerate_graph(&graph);
        let expected = vec![vec![], vec![2, 3], vec![1, 3, 4], vec![1, 2], vec![2]];

        result.iter_mut().for_each(|v| v.sort_unstable());
        assert_eq!(result, expected);
    }

    #[test]
    fn integer_vertices() {
        let mut graph = Graph::new();
        add_edge(&mut graph, 1001, 1002);
        add_edge(&mut graph, 1002, 1003);
        add_edge(&mut graph, 1003, 1001);
        add_edge(&mut graph, 1004, 1002);
        let mut result = enumerate_graph(&graph);
        let expected = vec![vec![], vec![2, 3], vec![1, 3, 4], vec![1, 2], vec![2]];

        result.iter_mut().for_each(|v| v.sort_unstable());
        assert_eq!(result, expected);
    }
}
