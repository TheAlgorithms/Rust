/// Module that contains the datastructures to represent a graph.
/// This is represented as an array of Nodes where each node contains
/// a vector of indices that the node is connected to. The graph is
/// represented as a directed graph.

/// An individual node of the graph that contains it's own index in the
/// array, a list of indices that it has a directed edge to and a field
/// with value of the type T.
#[derive(Debug)]
pub struct Node<T> {
    pub id: usize,
    pub edges: Vec<usize>,
    pub val: T,
}

/// The container that holds the actual list of Nodes. The relationship
/// between each node is defined within the node.
pub struct Graph<T> {
    pub g: Vec<Node<T>>,
}

impl<T: Copy + Clone> Node<T> {
    /// Returns a new node with a value of type T. This node is a floating
    /// node and is not linked to any other node at the moment. It has to
    /// be assinged it's own index in the graph and then the links.
    pub fn new(val: T) -> Node<T> {
        Self {
            id: 0,
            edges: Vec::new(),
            val: val,
        }
    }
}

impl<T> Graph<T> {
    /// Create an empty graph and returns it.
    pub fn new() -> Graph<T> {
        Self { g: Vec::new() }
    }

    /// Push a new node and consumes it. The ownership of the node will be
    /// taken by the graph. This function will assign the index to the node
    /// that is pushed.
    pub fn push(&mut self, mut node: Node<T>) {
        let sz = self.g.len();
        node.id = sz;
        self.g.push(node);
    }

    /// Creates a directed edge between 2 nodes in the graph. It will simply
    /// add an entry in the `from` node that points to the `to` node. To crate
    /// a undirected graph, the called can push an edge to both from and to
    /// nodes individually.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.g[from].edges.push(to);
    }
}
