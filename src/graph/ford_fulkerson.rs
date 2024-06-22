/*
The Ford-Fulkerson algorithm is a widely used algorithm to solve the maximum flow problem in a flow network. The maximum flow problem involves determining the maximum amount of flow that can be sent from a source vertex to a sink vertex in a directed weighted graph, subject to capacity constraints on the edges.

The following is simple idea of Ford-Fulkerson algorithm:

   1. Start with initial flow as 0.
   2. While there exists an augmenting path from the source to the sink:
       i. Find an augmenting path using any path-finding algorithm, such as breadth-first search or depth-first search.

       ii. Determine the amount of flow that can be sent along the augmenting path, which is the minimum residual capacity along the edges of the path.

       iii. Increase the flow along the augmenting path by the determined amount.
    3.Return the maximum flow.

*/
use std::collections::VecDeque;

const V: usize = 6; // Number of vertices in graph

pub fn bfs(r_graph: &[Vec<i32>], s: usize, t: usize, parent: &mut [i32]) -> bool {
    let mut visited = [false; V];
    visited[s] = true;
    parent[s] = -1;

    let mut queue = VecDeque::new();
    queue.push_back(s);

    while let Some(u) = queue.pop_front() {
        for v in 0..V {
            if !visited[v] && r_graph[u][v] > 0 {
                visited[v] = true;
                parent[v] = u as i32; // Convert u to i32
                if v == t {
                    return true;
                }
                queue.push_back(v);
            }
        }
    }

    false
}

pub fn ford_fulkerson(graph: &[Vec<i32>], s: usize, t: usize) -> i32 {
    let mut r_graph = graph.to_owned();
    let mut parent = vec![-1; V];
    let mut max_flow = 0;

    while bfs(&r_graph, s, t, &mut parent) {
        let mut path_flow = i32::MAX;
        let mut v = t;

        while v != s {
            let u = parent[v] as usize;
            path_flow = path_flow.min(r_graph[u][v]);
            v = u;
        }

        v = t;
        while v != s {
            let u = parent[v] as usize;
            r_graph[u][v] -= path_flow;
            r_graph[v][u] += path_flow;
            v = u;
        }

        max_flow += path_flow;
    }

    max_flow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let graph = vec![
            vec![0, 12, 0, 13, 0, 0],
            vec![0, 0, 10, 0, 0, 0],
            vec![0, 0, 0, 13, 3, 15],
            vec![0, 0, 7, 0, 15, 0],
            vec![0, 0, 6, 0, 0, 17],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(ford_fulkerson(&graph, 0, 5), 23);
    }

    #[test]
    fn test_example_2() {
        let graph = vec![
            vec![0, 4, 0, 3, 0, 0],
            vec![0, 0, 4, 0, 8, 0],
            vec![0, 0, 0, 3, 0, 2],
            vec![0, 0, 0, 0, 6, 0],
            vec![0, 0, 6, 0, 0, 6],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(ford_fulkerson(&graph, 0, 5), 7);
    }

    #[test]
    fn test_example_3() {
        let graph = vec![
            vec![0, 10, 0, 10, 0, 0],
            vec![0, 0, 4, 2, 8, 0],
            vec![0, 0, 0, 0, 0, 10],
            vec![0, 0, 0, 0, 9, 0],
            vec![0, 0, 6, 0, 0, 10],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(ford_fulkerson(&graph, 0, 5), 19);
    }

    #[test]
    fn test_example_4() {
        let graph = vec![
            vec![0, 8, 0, 0, 3, 0],
            vec![0, 0, 9, 0, 0, 0],
            vec![0, 0, 0, 0, 7, 2],
            vec![0, 0, 0, 0, 0, 5],
            vec![0, 0, 7, 4, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(ford_fulkerson(&graph, 0, 5), 6);
    }

    #[test]
    fn test_example_5() {
        let graph = vec![
            vec![0, 16, 13, 0, 0, 0],
            vec![0, 0, 10, 12, 0, 0],
            vec![0, 4, 0, 0, 14, 0],
            vec![0, 0, 9, 0, 0, 20],
            vec![0, 0, 0, 7, 0, 4],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(ford_fulkerson(&graph, 0, 5), 23);
    }
}
