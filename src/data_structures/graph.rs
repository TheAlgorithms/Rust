// Implemented by any struct that can be searched like a graph
pub trait SearchableGraph {
    // Number of nodes in the graph
    fn num_nodes(&self) -> usize;

    // Returns list of nodes adjacent to a base node
    fn neighbours(&self, node: usize) -> Vec<usize>;
}

// Graph edges stored in an adjacency matrix
pub struct MatrixGraph<T> {
    edges: Vec<Vec<T>>,
}

impl<T> MatrixGraph<T> {
    pub fn new(edges: Vec<Vec<T>>) -> Self {
        MatrixGraph { edges }
    }
}

impl<T: PartialOrd + From<usize>> SearchableGraph for MatrixGraph<T> {
    fn num_nodes(&self) -> usize {
        self.edges.len()
    }

    fn neighbours(&self, node: usize) -> Vec<usize> {
        self.edges[node]
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if *x > T::from(0) { Some(i) } else { None })
            .collect()
    }
}

// TODO: implement weighted graph and flow graph
// TODO: implement ListGraph, a graph stored as an adjacency list
