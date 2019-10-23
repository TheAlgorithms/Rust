/// Floyd-Warshall's Alogrithm for All Pairs Shortest Path

/// Floyd-Warshall's Algorithm for All Pairs Shortest Path finds the
/// shortest distance inbetween each pair of nodes in a graph in O(V^3)
/// time where V is the number of vertices within the graph.
///
/// NOTE: For this implementation, a `NONE` value represents `INF`, or
/// when there is no path between two nodes.
///
type Graph = Vec<Vec<Option<i128>>>;
pub fn floyd_warshall(graph: &mut Graph) -> Graph {
    for mid in 0..graph.len() {
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                let a = graph[i][mid];
                let b = graph[mid][j];
                let c = graph[i][j];

                if a.is_some() && b.is_some() {
                    let a_val = a.unwrap();
                    let b_val = b.unwrap();
                    let new_val = a_val + b_val;

                    if c.is_some() {
                        if a_val + b_val < c.unwrap() {
                            graph[i][j] = Some(new_val);
                        }
                    } else {
                        graph[i][j] = Some(new_val);
                    }
                }
            }
        }
    }
    graph.to_vec()
}

#[cfg(test)]
mod tests {
    use super::floyd_warshall;
    use super::Graph;

    #[test]
    fn test_floyd_warshall() {
        let mut input: Graph = vec![
            vec![Some(0), None, Some(-2), None],
            vec![Some(4), Some(0), Some(3), None],
            vec![None, None, Some(0), Some(2)],
            vec![None, Some(-1), None, Some(0)],
        ];

        let expect: Graph = vec![
            vec![Some(0), Some(-1), Some(-2), Some(0)],
            vec![Some(4), Some(0), Some(2), Some(4)],
            vec![Some(5), Some(1), Some(0), Some(2)],
            vec![Some(3), Some(-1), Some(1), Some(0)],
        ];

        let output: Graph = floyd_warshall(&mut input);
        assert_eq!(output, expect);
    }

    #[test]
    fn test_with_infinite_path() {
        let mut input: Graph = vec![
            vec![Some(0), Some(1), Some(43)],
            vec![Some(1), Some(0), Some(6)],
            vec![None, None, Some(0)],
        ];

        let expect: Graph = vec![
            vec![Some(0), Some(1), Some(7)],
            vec![Some(1), Some(0), Some(6)],
            vec![None, None, Some(0)],
        ];

        let output: Graph = floyd_warshall(&mut input);
        assert_eq!(output, expect);
    }
}
