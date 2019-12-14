use std::collections::HashMap;

// This trait defines what Graphs should DO.
trait _Graph {
    // Static method signature; `Self` refers to the implementor type.
    fn new() -> Self;

    // Instance function to add new nodes to the Graph.
    // @param &mut  self     Reference for instance
    // @param Tuple nodes    Tuple with edge values.
    fn add(&mut self, src: usize, des: usize);

    // Returns number of Nodes added to the Graph.
    fn get_number_of_nodes(&mut self) -> usize;

    // Returns number of Edges added to the Graph.
    fn get_number_of_edges(&mut self) -> usize;
}

// This struct defines what Graphs should BE.
pub struct Graph {
    number_of_nodes: usize,
    number_of_edges: usize,
    nodes: HashMap<usize, Vec<usize>>,
}

// Here we define an Undirected Graph
// pub struct UnGraph;

impl _Graph for Graph {
    fn new() -> Self {
        Self {
            number_of_nodes: 0,
            number_of_edges: 0,
            nodes: HashMap::new(),
        }
    }

    fn add(&mut self, src: usize, dest: usize) {
        // check if keys are on the graph
        if self.nodes.contains_key(&src) == false {
            self.number_of_nodes += 1;
            self.nodes.insert(src, Vec::new());
        }
        if self.nodes.contains_key(&dest) == false {
            self.number_of_nodes += 1;
            self.nodes.insert(dest, Vec::new());
        }

        // as it is an undirected graph, insert both edges (e, -e)
        self.nodes.entry(src).or_insert(Vec::new()).push(dest);
        self.nodes.entry(dest).or_insert(Vec::new()).push(dest);
        self.number_of_edges += 2;
    }

    fn get_number_of_nodes(&mut self) -> usize {
        self.number_of_nodes
    }

    fn get_number_of_edges(&mut self) -> usize {
        self.number_of_edges
    }
}

pub struct DiGraph {
    number_of_nodes: usize,
    number_of_edges: usize,
    nodes: HashMap<usize, Vec<usize>>,
}

// A directed graph has the same funcionality as a graph,
// the only difference being the insert method.
impl _Graph for DiGraph {
    fn new() -> Self {
        Self {
            number_of_nodes: 0,
            number_of_edges: 0,
            nodes: HashMap::new(),
        }
    }

    fn add(&mut self, src: usize, dest: usize) {
        if self.nodes.contains_key(&src) == false {
            self.number_of_nodes += 1;
            self.nodes.insert(src, Vec::new());
        }
        if self.nodes.contains_key(&dest) == false {
            self.number_of_nodes += 1;
        }

        // As it is a Digraph, we only inser one edge
        self.number_of_edges += 1;
        self.nodes.entry(src).or_insert(Vec::new()).push(dest);
    }

    fn get_number_of_nodes(&mut self) -> usize {
        self.number_of_nodes
    }

    fn get_number_of_edges(&mut self) -> usize {
        self.number_of_edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undirected_graph_add() {
        let mut undirected_graph = Graph::new();
        undirected_graph.add(0, 1);
        undirected_graph.add(0, 2);
        undirected_graph.add(1, 2);
        undirected_graph.add(2, 3);
        undirected_graph.add(1, 3);
        undirected_graph.add(3, 4);
        assert_eq!(undirected_graph.get_number_of_nodes(), 5);
        assert_eq!(undirected_graph.get_number_of_edges(), 12);
    }

    #[test]
    fn test_directed_graph_add() {
        let mut directed_graph = DiGraph::new();
        directed_graph.add(0, 1);
        directed_graph.add(0, 2);
        directed_graph.add(1, 2);
        directed_graph.add(2, 3);
        directed_graph.add(1, 3);
        directed_graph.add(3, 4);
        assert_eq!(directed_graph.get_number_of_nodes(), 5);
        assert_eq!(directed_graph.get_number_of_edges(), 6);
    }
}
