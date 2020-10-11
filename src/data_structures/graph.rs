use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Vertex<T> {
    value: Rc<T>,
}

impl<T> Vertex<T> {
    fn new(value: T) -> Self {
        Self {
            value: Rc::new(value),
        }
    }
}

pub trait Graph<T> {
    /// lists all vertices y such that there is an edge from the vertex x to the vertex y
    fn neighbors(&self, x: &Vertex<T>) -> Option<&Vec<Vertex<T>>>;

    /// tests whether there is an edge from the vertex x to the vertex y
    fn adjacent(&self, x: &Vertex<T>, y: &Vertex<T>) -> bool;

    /// adds the vertex x, if it is not there
    fn add_vertex(&mut self, v: &Vertex<T>) -> &mut Self;

    /// removes the vertex x, if it is there
    fn remove_vertex(&mut self, x: &Vertex<T>) -> &mut Self;

    /// adds the edge from the vertex x to the vertex y
    fn add_edge(&mut self, x: &Vertex<T>, y: &Vertex<T>) -> &mut Self;

    /// removes the edge from the vertex x to the vertex y, if it is there
    fn remove_edge(&mut self, x: &Vertex<T>, y: &Vertex<T>) -> &mut Self;
}

pub trait WeightedGraph<T, W>: Graph<T> {
    /// returns the value associated with the edge (x, y)
    fn get_edge_value(&self, x: &Vertex<T>, y: &Vertex<T>) -> Option<W>;

    /// sets the value associated with the edge (x, y) to v
    fn set_edge_value(&mut self, x: &Vertex<T>, y: &Vertex<T>, v: W);
}

pub struct AdjacencyList<T> {
    vertices: HashMap<Vertex<T>, Vec<Vertex<T>>>,
}

impl<T> AdjacencyList<T> {
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
        }
    }
}

impl<T> Graph<T> for AdjacencyList<T>
where
    T: Hash + Eq + Clone,
{
    /// lists all vertices y such that there is an edge from the vertex x to the vertex y
    fn neighbors(&self, x: &Vertex<T>) -> Option<&Vec<Vertex<T>>> {
        self.vertices.get(&x)
    }

    /// tests whether there is an edge from the vertex x to the vertex y
    fn adjacent(&self, x: &Vertex<T>, y: &Vertex<T>) -> bool {
        if let Some(neighbors) = self.vertices.get(&x) {
            neighbors.contains(&y)
        } else {
            false
        }
    }

    /// adds the vertex x, if it is not there
    fn add_vertex(&mut self, v: &Vertex<T>) -> &mut Self {
        self.vertices.insert(v.clone(), Vec::new());
        self
    }

    /// removes the vertex x, if it is there
    fn remove_vertex(&mut self, x: &Vertex<T>) -> &mut Self {
        if let Some(neighbors) = self.vertices.remove(&x) {
            // remove adjacencies
            for neighbor in &neighbors {
                if let Some(their_neighbors) = self.vertices.get_mut(&neighbor) {
                    if let Some(pos) = their_neighbors.iter().position(|i| i == x) {
                        their_neighbors.remove(pos);
                    }
                }
            }
        }

        self
    }

    /// adds the edge from the vertex x to the vertex y
    fn add_edge(&mut self, x: &Vertex<T>, y: &Vertex<T>) -> &mut Self {
        if self.vertices.contains_key(&x) && self.vertices.contains_key(&y) {
            // add y to x neighbors
            self.vertices
                .get_mut(&x)
                .expect("vertex always has Vec of neighbors")
                .push(y.clone());
            // add x to y neighbors
            self.vertices
                .get_mut(&y)
                .expect("vertex always has Vec of neighbors")
                .push(x.clone());
        }

        self
    }

    /// removes the edge from the vertex x to the vertex y, if it is there
    fn remove_edge(&mut self, x: &Vertex<T>, y: &Vertex<T>) -> &mut Self {
        if self.vertices.contains_key(&x) && self.vertices.contains_key(&y) {
            // remove y from x neighbors
            let neighbors = self
                .vertices
                .get_mut(&x)
                .expect("vertex always has Vec of neighbors");
            if let Some(pos) = neighbors.iter().position(|i| i == y) {
                neighbors.remove(pos);
            }
            // remove x from y neighbors
            let neighbors = self
                .vertices
                .get_mut(&y)
                .expect("vertex always has Vec of neighbors");
            if let Some(pos) = neighbors.iter().position(|i| i == x) {
                neighbors.remove(pos);
            }
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_graph_add_vertex() {
        let mut graph = AdjacencyList::new();
        graph.add_vertex(&Vertex::new("hello, world"));

        assert!(!graph.vertices.is_empty());
    }

    #[test]
    fn test_graph_remove_vertex() {
        let mut graph = AdjacencyList::new();
        let v = Vertex::new("add then remove");
        graph.add_vertex(&v).remove_vertex(&v);

        assert!(graph.vertices.is_empty());
    }

    #[test]
    fn test_graph_remove_vertex_removes_neighbors() {
        let mut graph = AdjacencyList::new();
        let v1 = Vertex::new(1);
        let v2 = Vertex::new(2);

        graph
            .add_vertex(&v1)
            .add_vertex(&v2)
            .add_edge(&v1, &v2)
            .remove_vertex(&v1);

        assert!(graph.neighbors(&v2).unwrap().is_empty());
    }

    #[test]
    fn test_graph_add_edge() {
        let mut graph = AdjacencyList::new();
        let v1 = Vertex::new("alone then connected then alone");
        let v2 = Vertex::new("also alone then connected then alone");

        graph.add_vertex(&v1).add_vertex(&v2);

        assert!(graph.neighbors(&v1).unwrap().is_empty());
        assert!(graph.neighbors(&v2).unwrap().is_empty());
        assert!(!graph.adjacent(&v1, &v2));

        graph.add_edge(&v1, &v2);

        assert_eq!(graph.neighbors(&v1).unwrap(), &vec![v2.clone()]);
        assert_eq!(graph.neighbors(&v2).unwrap(), &vec![v1.clone()]);
        assert!(graph.adjacent(&v1, &v2));
    }
}
