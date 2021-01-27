use std::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, HashMap<String, i32>>,
}

impl UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {adjacency_table: HashMap::new()}
    }

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacency_table.get(node) {
            None => {
                self.adjacency_table.insert((*node).to_string(), HashMap::new());
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
            e.insert(edge.1.to_string(), edge.2);
        });
        self.adjacency_table
        .entry(edge.1.to_string())
        .and_modify(|e| {
            e.insert(edge.0.to_string(), edge.2);
        });
    }

    fn neighbours(&self, node: &str) -> Result<Vec<(&str, i32)>, NodeNotInGraph>{
        match self.adjacency_table.get(node) {
            None => {
                return Err(NodeNotInGraph)
            },
            Some(i) => {
                let mut neighbours: Vec<(&str, i32)> = Vec::new();
                for (key, value) in i {
                    neighbours.push((key, *value))
                }
                Ok(neighbours)
            }
        }
    }

    fn contains(&self, node: &str) -> bool {
        match self.adjacency_table.get(node) {
            None => {
                return false
            },
            _ => {
                return true
            }
        }
    }

    fn nodes(&self) -> HashSet<&String> {
        let mut nodes = HashSet::new();
        for (key, _) in &self.adjacency_table {
            nodes.insert(key);
        }
        nodes
    }

    fn edges(&self) -> HashSet<(&String, &String, i32)> {
        let mut edges = HashSet::new();
        for (from_node, from_node_neighbours) in &self.adjacency_table {
            for (to_node, weight) in from_node_neighbours {
                edges.insert((from_node, to_node, *weight));
            }
        }
        edges
    }
}


#[cfg(test)]
mod test {
    use super::UndirectedGraph;

    #[test]
    fn test_add_node() {
        let mut ugraph = UndirectedGraph::new();
        ugraph.add_node("a");
        ugraph.add_node("b");
        ugraph.add_node("c");
        assert_eq!(ugraph.nodes(), [&String::from("a"), &String::from("b"), &String::from("c")].iter().cloned().collect());
    }

    #[test]
    fn test_add_edge() {
        let mut ugraph = UndirectedGraph::new();
        
        ugraph.add_edge(("a", "b", 5));
        ugraph.add_edge(("b", "c", 10));
        ugraph.add_edge(("c", "a", 7));

        assert_eq!(ugraph.edges(), [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7)
        ].iter().cloned().collect());
    }

    #[test]
    fn test_neighbours() {
        let mut ugraph = UndirectedGraph::new();

        ugraph.add_edge(("a", "b", 5));
        ugraph.add_edge(("b", "c", 10));
        ugraph.add_edge(("c", "a", 7));

        assert_eq!(ugraph.neighbours("a").unwrap(), vec![("b", 5), ("c", 7)]);
    }

    #[test]
    fn test_contains() {
        let mut ugraph = UndirectedGraph::new();
        ugraph.add_node("a");
        ugraph.add_node("b");
        ugraph.add_node("c");
        assert_eq!(ugraph.contains("a"), true);
        assert_eq!(ugraph.contains("b"), true);
        assert_eq!(ugraph.contains("c"), true);
        assert_eq!(ugraph.contains("d"), false);
    }
}
