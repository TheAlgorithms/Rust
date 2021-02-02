use std::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct Graph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {adjacency_table: HashMap::new()}
    }

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacency_table.get(node) {
            None => {
                self.adjacency_table.insert((*node).to_string(), Vec::new());
                return true
            },
            _ => {
                return false
            }
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table
        .entry(edge.0.to_string())
        .and_modify(|e| {
            e.push((edge.1.to_string(), edge.2));
        });
        self.adjacency_table
        .entry(edge.1.to_string())
        .and_modify(|e| {
            e.push((edge.0.to_string(), edge.2));
        });
    }

    fn neighbours(&self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph>{
        match self.adjacency_table.get(node) {
            None => {
                return Err(NodeNotInGraph)
            },
            Some(i) => { Ok(i) }
        }
    }

    fn contains(&self, node: &str) -> bool {self.adjacency_table.get(node).is_some()}

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table.keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in &self.adjacency_table {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}


#[cfg(test)]
mod test {
    use super::Graph;

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(graph.nodes(), [&String::from("a"), &String::from("b"), &String::from("c")].iter().cloned().collect());
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = Graph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(graph.neighbours("a").unwrap(), &vec![(String::from("b"), 5), (String::from("c"), 7)]);
    }

    #[test]
    fn test_contains() {
        let mut graph = Graph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(graph.contains("a"), true);
        assert_eq!(graph.contains("b"), true);
        assert_eq!(graph.contains("c"), true);
        assert_eq!(graph.contains("d"), false);
    }
}
