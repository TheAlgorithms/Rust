use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// # Undirected Graph
///
/// Used to build a graph whose edges don't have a **specific** direction.
///
/// A given graph consists of vertices A, B and C. There are edges connecting
/// - A and B
/// - B and C
///
/// This means vertex A is adjacent to vertex B but not to vertex C. The vertex B
/// is adjacent to the vertices A and C.
///
/// ```
/// use the_algorithms_rust::data_structures::{UnDiGraph,Graph};
///
/// let mut graph = UnDiGraph::<&'static str, i32>::default();
/// graph.add_vertex("A", 10);
/// graph.add_vertex("B", 11);
/// graph.add_vertex("C", 9);
/// graph.add_edge("A", "B");
/// graph.add_edge("B", "C");
///
/// assert!(graph.adjacent("A", "B"));
/// assert!(graph.adjacent("B", "A"));
/// assert!(!graph.adjacent("A", "C"));
/// ```
///
/// For more information see
/// [https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)#Graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)#Graph)
pub struct UnDiGraph<Node, ValueType> {
    /// Vertices as the node types/names and its associated value
    vertices: HashMap<Node, ValueType>,

    /// The edges between the vertices
    edges: HashMap<Node, HashSet<Node>>,
}

/// Directed Graph
///
/// Used to build a graph whose edges have a **specific** direction.
///
/// A given graph consists of vertices A, B and C. There are edges connecting
/// - A and B
/// - B and A
/// - B and C
///
/// This means vertex A is adjacent to vertex B but not to vertex C. The vertex B
/// is adjacent to the vertices A and C. C on the other hand is not adjacent to
/// the vertices B or A.
///
/// ```
/// use the_algorithms_rust::data_structures::{DiGraph,Graph};
///
/// let mut graph = DiGraph::<&'static str, i32>::default();
/// graph.add_vertex("A", 10);
/// graph.add_vertex("B", 11);
/// graph.add_vertex("C", 9);
/// graph.add_edge("A", "B");
/// graph.add_edge("B", "C");
///
/// assert!(graph.adjacent("A", "B"));
/// assert!(graph.adjacent("B", "A"));
/// assert!(!graph.adjacent("A", "C"));
/// ```
///
///
/// For more information see
/// - [https://en.wikipedia.org/wiki/Directed_graph](https://en.wikipedia.org/wiki/Directed_graph)
/// - [https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)#Directed_graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)#Directed_graph)
pub struct DiGraph<Node, ValueType> {
    /// Vertices as the node types/names and its associated value
    vertices: HashMap<Node, ValueType>,

    /// The edges between the vertices
    edges: HashMap<Node, HashSet<Node>>,
}

/// General description of a graph with its operations
pub trait Graph<Node, ValueType> {
    /// Tests whether there is an edge from the vertex source to the vertex
    /// target
    fn adjacent(&self, source: Node, target: Node) -> bool;

    /// Lists all vertices y such that there is an edge from the vertex
    /// source to the vertex y
    fn neighbours(&self, source: Node) -> Vec<&Node>;

    /// Add a vertex to the graph
    fn add_vertex(&mut self, node: Node, value: ValueType);

    /// Remove a vertex from the graph
    fn remove_vertex(&mut self, node: Node);

    /// Add an edge between the node source and the node target
    fn add_edge(&mut self, source: Node, target: Node);

    /// Remove the edge between the node source and the node target
    fn remove_edge(&mut self, source: Node, target: Node);

    /// Get the associated value of a node
    fn get_vertex_value(&self, node: Node) -> Option<ValueType>;

    /// Set the associated value of a node
    fn set_vertex_value(&mut self, node: Node, value: ValueType);

    // TODO: do we want to have associated value to an edge?
    // if yes then we may want to add following functions
    // fn get_edge_value(&self, source: Node, target: Node) -> EdgeValueType;
    // fn set_edge_value(&mut self, source: Node, target: Node, value: EdgeValueType);
}

impl<Node, ValueType> UnDiGraph<Node, ValueType> {
    /// Construct a new undirected Graph
    pub fn new() -> Self {
        UnDiGraph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<Node, ValueType> Default for UnDiGraph<Node, ValueType> {
    /// Return a new empty undirected graph
    fn default() -> Self {
        Self::new()
    }
}

impl<Node, ValueType> Graph<Node, ValueType> for UnDiGraph<Node, ValueType>
    where
        Node: Ord + Hash + Clone,
        ValueType: Copy + Clone,
{
    /// Check if a vertex source has an edge to the vertex target
    ///
    /// ```
    /// use the_algorithms_rust::data_structures::{UnDiGraph,Graph};
    ///
    /// let mut graph = UnDiGraph::<i32, i32>::default();
    /// graph.add_vertex(1, 10);
    /// graph.add_vertex(2, 11);
    /// graph.add_vertex(3, 9);
    /// graph.add_edge(1, 2);
    ///
    /// assert!(graph.adjacent(1, 2));
    /// assert!(graph.adjacent(2, 1));
    /// assert!(!graph.adjacent(1, 3));
    /// ```
    fn adjacent(&self, source: Node, target: Node) -> bool {
        for (vert_source, vert_targets) in &self.edges {
            if *vert_source != source && *vert_source != target {
                continue;
            }
            for vert_target in vert_targets {
                if (*vert_source == source && *vert_target == target)
                    || (*vert_source == target && *vert_target == source)
                {
                    return true;
                }
            }
        }
        false
    }

    fn neighbours(&self, source: Node) -> Vec<&Node> {
        let mut neighbours = Vec::new();
        if let Some(edges) = self.edges.get(&source) {
            neighbours.extend(edges);
        }
        // TODO: other direction
        neighbours
    }

    fn add_vertex(&mut self, node: Node, value: ValueType) {
        self.vertices.insert(node, value);
    }

    fn remove_vertex(&mut self, node: Node) {
        self.vertices.remove(&node);
    }

    fn add_edge(&mut self, source: Node, target: Node) {
        let s = source.clone();
        if !self.edges.contains_key(&source) {
            self.edges.insert(s, HashSet::new());
        }
        if let Some(edges) = &mut self.edges.get_mut(&source) {
            edges.insert(target);
        }
    }

    fn remove_edge(&mut self, source: Node, target: Node) {
        if let Some(edges) = self.edges.get_mut(&source)
        {
            edges.remove(&target);
        }
    }

    fn get_vertex_value(&self, node: Node) -> Option<ValueType> {
        if !self.vertices.contains_key(&node) {
            None
        } else {
            Some(self.vertices[&node])
        }
    }

    fn set_vertex_value(&mut self, node: Node, value: ValueType) {
        if let Some(vert_value) = self.vertices.get_mut(&node) {
            *vert_value = value;
        }
    }
}

impl<Node, ValueType> DiGraph<Node, ValueType> {
    /// Construct a new undirected Graph
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<Node, ValueType> Default for DiGraph<Node, ValueType> {
    /// Return a new empty undirected graph
    fn default() -> Self {
        Self::new()
    }
}

impl<Node, ValueType> Graph<Node, ValueType> for DiGraph<Node, ValueType>
    where
        Node: Ord + Hash + Clone,
        ValueType: Copy + Clone,
{
    fn adjacent(&self, source: Node, target: Node) -> bool {
        unimplemented!()
    }

    fn neighbours(&self, source: Node) -> Vec<&Node> {
        unimplemented!()
    }

    fn add_vertex(&mut self, node: Node, value: ValueType) {
        unimplemented!()
    }

    fn remove_vertex(&mut self, node: Node) {
        unimplemented!()
    }

    fn add_edge(&mut self, source: Node, target: Node) {
        unimplemented!()
    }

    fn remove_edge(&mut self, source: Node, target: Node) {
        unimplemented!()
    }

    fn get_vertex_value(&self, node: Node) -> Option<ValueType> {
        unimplemented!()
    }

    fn set_vertex_value(&mut self, node: Node, value: ValueType) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::{DiGraph, Graph, UnDiGraph};
}


