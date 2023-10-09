// Kosaraju algorithm, a linear-time algorithm to find the strongly connected components (SCCs) of a directed graph, in Rust.
pub struct Graph {
    vertices: usize,
    adj_list: Vec<Vec<usize>>,
    transpose_adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adj_list: vec![vec![]; vertices],
            transpose_adj_list: vec![vec![]; vertices],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.transpose_adj_list[v].push(u);
    }

    pub fn dfs(&self, node: usize, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
        visited[node] = true;
        for &neighbor in &self.adj_list[node] {
            if !visited[neighbor] {
                self.dfs(neighbor, visited, stack);
            }
        }
        stack.push(node);
    }

    pub fn dfs_scc(&self, node: usize, visited: &mut Vec<bool>, scc: &mut Vec<usize>) {
        visited[node] = true;
        scc.push(node);
        for &neighbor in &self.transpose_adj_list[node] {
            if !visited[neighbor] {
                self.dfs_scc(neighbor, visited, scc);
            }
        }
    }
}

pub fn kosaraju(graph: &Graph) -> Vec<Vec<usize>> {
    let mut visited = vec![false; graph.vertices];
    let mut stack = Vec::new();

    for i in 0..graph.vertices {
        if !visited[i] {
            graph.dfs(i, &mut visited, &mut stack);
        }
    }

    let mut sccs = Vec::new();
    visited = vec![false; graph.vertices];

    while let Some(node) = stack.pop() {
        if !visited[node] {
            let mut scc = Vec::new();
            graph.dfs_scc(node, &mut visited, &mut scc);
            sccs.push(scc);
        }
    }

    sccs
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kosaraju_single_sccs() {
        let vertices = 5;
        let mut graph = Graph::new(vertices);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 0);
        graph.add_edge(4, 2);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 1);
        assert!(sccs.contains(&vec![0, 3, 2, 1, 4]));
    }

    #[test]
    fn test_kosaraju_multiple_sccs() {
        let vertices = 8;
        let mut graph = Graph::new(vertices);

        graph.add_edge(1, 0);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 6);
        graph.add_edge(6, 7);
        graph.add_edge(4, 7);
        graph.add_edge(6, 4);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 4);
        assert!(sccs.contains(&vec![0, 1, 2]));
        assert!(sccs.contains(&vec![3]));
        assert!(sccs.contains(&vec![4, 6, 5]));
        assert!(sccs.contains(&vec![7]));
    }

    #[test]
    fn test_kosaraju_multiple_sccs1() {
        let vertices = 8;
        let mut graph = Graph::new(vertices);
        graph.add_edge(0, 2);
        graph.add_edge(1, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 7);
        graph.add_edge(5, 2);
        graph.add_edge(5, 6);
        graph.add_edge(6, 5);
        graph.add_edge(7, 6);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 3);
        assert!(sccs.contains(&vec![0]));
        assert!(sccs.contains(&vec![1]));
        assert!(sccs.contains(&vec![2, 5, 6, 7, 4, 3]));
    }

    #[test]
    fn test_kosaraju_no_scc() {
        let vertices = 4;
        let mut graph = Graph::new(vertices);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 4);
        for (i, _) in sccs.iter().enumerate().take(vertices) {
            assert_eq!(sccs[i], vec![i]);
        }
    }

    #[test]
    fn test_kosaraju_empty_graph() {
        let vertices = 0;
        let graph = Graph::new(vertices);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 0);
    }
}
