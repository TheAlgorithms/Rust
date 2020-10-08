#![allow(dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq)]
struct Vertex<T>
where
    T: Hash + Eq,
{
    value: T,
}

trait Graph<T>
where
    T: Hash + Eq,
{
    /// lists all vertices y such that there is an edge from the vertex x to the vertex y
    fn neighbors(&self, x: Vertex<T>) -> Option<&Vec<Vertex<T>>>;

    /// tests whether there is an edge from the vertex x to the vertex y
    fn adjacent(&self, x: Vertex<T>, y: Vertex<T>) -> bool;

    /// adds the vertex x, if it is not there
    fn add_vertex(&mut self, v: Vertex<T>) -> &mut Self;

    /// removes the vertex x, if it is there
    fn remove_vertex(&mut self, x: Vertex<T>) -> &mut Self;

    /// adds the edge from the vertex x to the vertex y
    fn add_edge(&mut self, x: Vertex<T>, y: Vertex<T>) -> &mut Self;

    /// removes the edge from the vertex x to the vertex y, if it is there
    fn remove_edge(&mut self, x: Vertex<T>, y: Vertex<T>) -> &mut Self;
}

trait WeightedGraph<T, W>: Graph<T>
where
    T: Hash + Eq,
{
    /// returns the value associated with the edge (x, y)
    fn get_edge_value(&self, x: Vertex<T>, y: Vertex<T>) -> W;

    /// sets the value associated with the edge (x, y) to v
    fn set_edge_value(&mut self, x: Vertex<T>, y: Vertex<T>, v: W);
}

struct AdjacencyList<T>
where
    T: Hash + Eq,
{
    vertices: HashMap<Vertex<T>, Vec<Vertex<T>>>,
}

impl<T> Graph<T> for AdjacencyList<T>
where
    T: Hash + Eq,
{
    /// lists all vertices y such that there is an edge from the vertex x to the vertex y
    fn neighbors(&self, x: Vertex<T>) -> Option<&Vec<Vertex<T>>> {
        self.vertices.get(&x)
    }

    /// tests whether there is an edge from the vertex x to the vertex y
    fn adjacent(&self, x: Vertex<T>, y: Vertex<T>) -> bool {
        todo!()
    }

    /// adds the vertex x, if it is not there
    fn add_vertex(&mut self, v: Vertex<T>) -> &mut Self {
        todo!()
    }

    /// removes the vertex x, if it is there
    fn remove_vertex(&mut self, x: Vertex<T>) -> &mut Self {
        todo!()
    }

    /// adds the edge from the vertex x to the vertex y
    fn add_edge(&mut self, x: Vertex<T>, y: Vertex<T>) -> &mut Self {
        todo!()
    }

    /// removes the edge from the vertex x to the vertex y, if it is there
    fn remove_edge(&mut self, x: Vertex<T>, y: Vertex<T>) -> &mut Self {
        todo!()
    }
}
