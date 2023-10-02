use std::collections::LinkedList;
use std::vec::Vec;

/// Struct representing an Eulerian Path in a directed graph.
pub struct EulerianPath {
    n: usize,                // Number of nodes in the graph
    edge_count: usize,       // Total number of edges in the graph
    in_degree: Vec<usize>,   // In-degrees of nodes
    out_degree: Vec<usize>,  // Out-degrees of nodes
    path: LinkedList<usize>, // Linked list to store the Eulerian path
    graph: Vec<Vec<usize>>,  // Adjacency list representing the directed graph
}

impl EulerianPath {
    /// Creates a new instance of EulerianPath.
    ///
    /// # Arguments
    ///
    /// * `graph` - A directed graph represented as an adjacency list.
    ///
    /// # Returns
    ///
    /// A new EulerianPath instance.
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            n,
            edge_count: 0,
            in_degree: vec![0; n],
            out_degree: vec![0; n],
            path: LinkedList::new(),
            graph,
        }
    }

    /// Finds an Eulerian path in the directed graph.
    ///
    /// # Returns
    ///
    /// An `Option` containing the Eulerian path if it exists, or `None` if no Eulerian path exists.
    pub fn find_eulerian_path(&mut self) -> Option<Vec<usize>> {
        self.initialize();

        if !self.has_eulerian_path() {
            return None;
        }

        let start_node = self.find_start_node();
        self.traverse(start_node);

        if self.path.len() != self.edge_count + 1 {
            return None;
        }

        let mut solution = Vec::with_capacity(self.edge_count + 1);
        while let Some(node) = self.path.pop_front() {
            solution.push(node);
        }

        Some(solution)
    }

    /// Initializes the degree vectors and counts the total number of edges in the graph.
    fn initialize(&mut self) {
        for (from, neighbors) in self.graph.iter().enumerate() {
            for &to in neighbors {
                self.in_degree[to] += 1;
                self.out_degree[from] += 1;
                self.edge_count += 1;
            }
        }
    }

    /// Checks if the graph has an Eulerian path.
    ///
    /// # Returns
    ///
    /// `true` if an Eulerian path exists, `false` otherwise.
    fn has_eulerian_path(&self) -> bool {
        if self.edge_count == 0 {
            return false;
        }

        let (mut start_nodes, mut end_nodes) = (0, 0);
        for i in 0..self.n {
            let in_degree = self.in_degree[i] as i32;
            let out_degree = self.out_degree[i] as i32;

            if (out_degree - in_degree) > 1 || (in_degree - out_degree) > 1 {
                return false;
            } else if (out_degree - in_degree) == 1 {
                start_nodes += 1;
            } else if (in_degree - out_degree) == 1 {
                end_nodes += 1;
            }
        }

        (end_nodes == 0 && start_nodes == 0) || (end_nodes == 1 && start_nodes == 1)
    }

    /// Finds the starting node for the Eulerian path.
    ///
    /// # Returns
    ///
    /// The index of the starting node.
    fn find_start_node(&self) -> usize {
        let mut start = 0;
        for i in 0..self.n {
            if self.out_degree[i] - self.in_degree[i] == 1 {
                return i;
            }
            if self.out_degree[i] > 0 {
                start = i;
            }
        }
        start
    }

    /// Traverses the graph to find the Eulerian path recursively.
    ///
    /// # Arguments
    ///
    /// * `at` - The current node being traversed.
    fn traverse(&mut self, at: usize) {
        while self.out_degree[at] != 0 {
            let next = self.graph[at][self.out_degree[at] - 1];
            self.out_degree[at] -= 1;
            self.traverse(next);
        }
        self.path.push_front(at);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates an empty graph with `n` nodes.
    fn create_empty_graph(n: usize) -> Vec<Vec<usize>> {
        vec![Vec::new(); n]
    }

    /// Adds a directed edge from `from` to `to` in the graph.
    fn add_directed_edge(graph: &mut [Vec<usize>], from: usize, to: usize) {
        graph[from].push(to);
    }

    #[test]
    fn good_path_test() {
        let n = 7;
        let mut graph = create_empty_graph(n);

        add_directed_edge(&mut graph, 1, 2);
        add_directed_edge(&mut graph, 1, 3);
        add_directed_edge(&mut graph, 2, 2);
        add_directed_edge(&mut graph, 2, 4);
        add_directed_edge(&mut graph, 2, 4);
        add_directed_edge(&mut graph, 3, 1);
        add_directed_edge(&mut graph, 3, 2);
        add_directed_edge(&mut graph, 3, 5);
        add_directed_edge(&mut graph, 4, 3);
        add_directed_edge(&mut graph, 4, 6);
        add_directed_edge(&mut graph, 5, 6);
        add_directed_edge(&mut graph, 6, 3);

        let mut solver = EulerianPath::new(graph);

        assert_eq!(
            solver.find_eulerian_path().unwrap(),
            vec![1, 3, 5, 6, 3, 2, 4, 3, 1, 2, 2, 4, 6]
        );
    }

    #[test]
    fn small_path_test() {
        let n = 5;
        let mut graph = create_empty_graph(n);

        add_directed_edge(&mut graph, 0, 1);
        add_directed_edge(&mut graph, 1, 2);
        add_directed_edge(&mut graph, 1, 4);
        add_directed_edge(&mut graph, 1, 3);
        add_directed_edge(&mut graph, 2, 1);
        add_directed_edge(&mut graph, 4, 1);

        let mut solver = EulerianPath::new(graph);

        assert_eq!(
            solver.find_eulerian_path().unwrap(),
            vec![0, 1, 4, 1, 2, 1, 3]
        );
    }
}
