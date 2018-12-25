use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

/// 'Graph<N, E, Ty>' is a data structure for graphs with optionally
/// weighted nodes and edges of arbitrary type. This implementation
/// is inteded for educational purposes only. 
/// See [petgraph](https://docs.rs/crate/petgraph/) for a full featured
/// implementation.
pub struct Graph<N, E, Ty = Directed> {
    nodes: HashMap<usize, Node<N>>,
    edges: HashMap<usize, Edge<E>>,
    ty: PhantomData<Ty>,
}

impl<N, E, Ty> Graph<N, E, Ty>
where
    Ty: EdgeType,
{
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            ty: PhantomData,
        }
    }

    /// Returns true iff graph is directed.
    fn is_directed(&self) -> bool {
        Ty::is_directed()
    }

    /// Add node to graph and return its assigned index.
    fn add_node(&mut self, weight: N) -> usize {
        let node = Node { weight: weight };
        let mut index = 0usize;
        while self.nodes.contains_key(&index) {
            index += 1;
        }
        self.nodes.insert(index, node);
        index
    }

    /// Given two indices head and tail, attempt to add an edge whose
    /// starting node has index head and whose ending node has index
    /// tail to the graph. If this succeeds, return index assigned to
    /// that edge. Otherwise return None.
    fn add_edge(&mut self, weight: E, head: usize, tail: usize) -> Option<usize> {
        if !self.nodes.contains_key(&head) || !self.nodes.contains_key(&head) {
            None
        } else {
            let mut index = 0usize;
            while self.edges.contains_key(&index) {
                index += 1;
            }
            self.edges.insert(
                index,
                Edge {
                    weight: weight,
                    head: head,
                    tail: tail,
                },
            );
            Some(index)
        }
    }

    /// Return first index of an edge of the form head --> tail.  This
    /// is not necessarily the unique index since we explicitly allow
    /// multiple edges between two nodes.
    fn find_edge(&self, head: usize, tail: usize) -> Option<usize> {
        for (index, edge) in self.edges.iter() {
            // The latter part of the following disjunction is needed
            // to identify [head,tail] with [tail,head] in undirected
            // graphs.
            if [edge.head, edge.tail] == [head, tail]
                || (self.is_directed() && [edge.head, edge.tail] == [tail, head])
            {
                return Some(*index);
            }
        }
        None
    }

    /// Return the indices of all edges of the form head --> tail
    fn find_edges(&self, head: usize, tail: usize) -> HashSet<usize> {
        let mut result: HashSet<usize> = HashSet::new();
        for (index, edge) in self.edges.iter() {
            if [edge.head, edge.tail] == [head, tail]
                || (self.is_directed() && [edge.head, edge.tail] == [tail, head])
            {
                result.insert(*index);
            }
        }
        result
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

/// The graph's node type.
pub struct Node<N> {
    // node data
    pub weight: N,
}

/// The graph's edge type.
#[derive(Debug, PartialEq)]
pub struct Edge<E> {
    // edge data
    pub weight: E,

    // indeces of head and tail
    head: usize,
    tail: usize,
}

pub trait EdgeType {
    fn is_directed() -> bool;
}

/// Marker for directed graphs
pub enum Directed {}

impl EdgeType for Directed {
    fn is_directed() -> bool {
        true
    }
}

/// Marker for undirected graphs
pub enum Undirected {}

impl EdgeType for Undirected {
    fn is_directed() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use data_structure::graph::*;

    #[test]
    fn add_node() {
        let mut graph: Graph<u32, u32> = Graph::new();
        // test if the new node has been given index 0
        assert_eq!(graph.add_node(5), 0);

        // test if there is a unique element in graph.nodes
        assert_eq!(graph.node_count(), 1);

        // test if graph.nodes contains a node with index 0
        assert_eq!(graph.nodes.contains_key(&0), true);

        // test if the node with index 0 has weight 5
        assert_eq!(graph.nodes.get(&0).unwrap().weight, 5);
    }

    #[test]
    fn add_edge() {
        let mut graph: Graph<(), String> = Graph::new();
        // Since we don't care about the weights of the nodes, we use
        // the unit type here. Unforunately, since there are no
        // optional arguments in Rust functions, we have to insert it
        // as an argument below as well.
        let start = graph.add_node(());
        let end = graph.add_node(());
        graph.add_edge(String::from("edge"), start, end);

        // Test if graph contains a unique edge.
        assert_eq!(graph.edge_count(), 1);

        // Test if graph's unique edge has weight 3.
        assert_eq!(graph.edges.get(&0).unwrap().weight, String::from("edge"));

        // Test if adding an illegal edge results in None
        assert_eq!(graph.add_edge(String::from("None"),2,3), None);
    }

    #[test]
    fn find_edge() {
        let mut graph: Graph<(), String> = Graph::new();
        let start_0 = graph.add_node(());
        let end_0 = graph.add_node(());
        let index_0 = graph
            .add_edge(String::from("edge 0"), start_0, end_0)
            .unwrap();

        let start_1 = graph.add_node(());
        let end_1 = graph.add_node(());
        let index_1 = graph
            .add_edge(String::from("edge 1"), start_1, end_1)
            .unwrap();

        assert_eq!(graph.find_edge(start_1, end_1).unwrap(), index_1);
        assert_eq!(graph.find_edge(start_0, end_0).unwrap(), index_0);
    }

    #[test]
    fn find_edges() {
        let mut graph: Graph<(), String> = Graph::new();
        let start = graph.add_node(());
        let end = graph.add_node(());
        let index_0 = graph.add_edge(String::from("edge 0"), start, end).unwrap();
        let index_1 = graph.add_edge(String::from("edge 1"), start, end).unwrap();

        // test if graph contains two distinct edges of the form 
        // start --> end
        let mut indices: HashSet<usize> = HashSet::new();
        indices.insert(index_0);
        indices.insert(index_1);
        assert_eq!(graph.find_edges(start, end), indices);
    }
}
