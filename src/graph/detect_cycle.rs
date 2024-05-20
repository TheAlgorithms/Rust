use std::collections::{HashMap, HashSet, VecDeque};

use crate::data_structures::{graph::Graph, DirectedGraph, UndirectedGraph};

pub trait DetectCycle {
    fn detect_cycle_dfs(&self) -> bool;
    fn detect_cycle_bfs(&self) -> bool;
}

// Helper function to detect cycle in an undirected graph using DFS graph traversal
fn undirected_graph_detect_cycle_dfs<'a>(
    graph: &'a UndirectedGraph,
    visited_node: &mut HashSet<&'a String>,
    parent: Option<&'a String>,
    u: &'a String,
) -> bool {
    visited_node.insert(u);
    for (v, _) in graph.adjacency_table().get(u).unwrap() {
        if matches!(parent, Some(parent) if v == parent) {
            continue;
        }
        if visited_node.contains(v)
            || undirected_graph_detect_cycle_dfs(graph, visited_node, Some(u), v)
        {
            return true;
        }
    }
    false
}

// Helper function to detect cycle in an undirected graph using BFS graph traversal
fn undirected_graph_detect_cycle_bfs<'a>(
    graph: &'a UndirectedGraph,
    visited_node: &mut HashSet<&'a String>,
    u: &'a String,
) -> bool {
    visited_node.insert(u);

    // Initialize the queue for BFS, storing (current node, parent node) tuples
    let mut queue = VecDeque::<(&String, Option<&String>)>::new();
    queue.push_back((u, None));

    while let Some((u, parent)) = queue.pop_front() {
        for (v, _) in graph.adjacency_table().get(u).unwrap() {
            if matches!(parent, Some(parent) if v == parent) {
                continue;
            }
            if visited_node.contains(v) {
                return true;
            }
            visited_node.insert(v);
            queue.push_back((v, Some(u)));
        }
    }
    false
}

impl DetectCycle for UndirectedGraph {
    fn detect_cycle_dfs(&self) -> bool {
        let mut visited_node = HashSet::<&String>::new();
        let adj = self.adjacency_table();
        for u in adj.keys() {
            if !visited_node.contains(u)
                && undirected_graph_detect_cycle_dfs(self, &mut visited_node, None, u)
            {
                return true;
            }
        }
        false
    }

    fn detect_cycle_bfs(&self) -> bool {
        let mut visited_node = HashSet::<&String>::new();
        let adj = self.adjacency_table();
        for u in adj.keys() {
            if !visited_node.contains(u)
                && undirected_graph_detect_cycle_bfs(self, &mut visited_node, u)
            {
                return true;
            }
        }
        false
    }
}

// Helper function to detect cycle in a directed graph using DFS graph traversal
fn directed_graph_detect_cycle_dfs<'a>(
    graph: &'a DirectedGraph,
    visited_node: &mut HashSet<&'a String>,
    in_stack_visited_node: &mut HashSet<&'a String>,
    u: &'a String,
) -> bool {
    visited_node.insert(u);
    in_stack_visited_node.insert(u);
    for (v, _) in graph.adjacency_table().get(u).unwrap() {
        if visited_node.contains(v) && in_stack_visited_node.contains(v) {
            return true;
        }
        if !visited_node.contains(v)
            && directed_graph_detect_cycle_dfs(graph, visited_node, in_stack_visited_node, v)
        {
            return true;
        }
    }
    in_stack_visited_node.remove(u);
    false
}

impl DetectCycle for DirectedGraph {
    fn detect_cycle_dfs(&self) -> bool {
        let mut visited_node = HashSet::<&String>::new();
        let mut in_stack_visited_node = HashSet::<&String>::new();
        let adj = self.adjacency_table();
        for u in adj.keys() {
            if !visited_node.contains(u)
                && directed_graph_detect_cycle_dfs(
                    self,
                    &mut visited_node,
                    &mut in_stack_visited_node,
                    u,
                )
            {
                return true;
            }
        }
        false
    }

    // detect cycle in a the graph using Kahn's algorithm
    // https://www.geeksforgeeks.org/detect-cycle-in-a-directed-graph-using-bfs/
    fn detect_cycle_bfs(&self) -> bool {
        // Set 0 in-degree for each vertex
        let mut in_degree: HashMap<&String, usize> =
            self.adjacency_table().keys().map(|k| (k, 0)).collect();

        // Calculate in-degree for each vertex
        for u in self.adjacency_table().keys() {
            for (v, _) in self.adjacency_table().get(u).unwrap() {
                *in_degree.get_mut(v).unwrap() += 1;
            }
        }
        // Initialize queue with vertex having 0 in-degree
        let mut queue: VecDeque<&String> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(&k, _)| k)
            .collect();

        let mut count = 0;
        while let Some(u) = queue.pop_front() {
            count += 1;
            for (v, _) in self.adjacency_table().get(u).unwrap() {
                in_degree.entry(v).and_modify(|d| {
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(v);
                    }
                });
            }
        }

        // If count of processed vertices is not equal to the number of vertices,
        // the graph has a cycle
        count != self.adjacency_table().len()
    }
}

#[cfg(test)]
mod test {
    use super::DetectCycle;
    use crate::data_structures::{graph::Graph, DirectedGraph, UndirectedGraph};
    fn get_undirected_single_node_with_loop() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "a", 1));
        res
    }
    fn get_directed_single_node_with_loop() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("a", "a", 1));
        res
    }
    fn get_undirected_two_nodes_connected() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res
    }
    fn get_directed_two_nodes_connected() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("b", "a", 1));
        res
    }
    fn get_directed_two_nodes() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res
    }
    fn get_undirected_triangle() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("c", "a", 1));
        res
    }
    fn get_directed_triangle() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("c", "a", 1));
        res
    }
    fn get_undirected_triangle_with_tail() -> UndirectedGraph {
        let mut res = get_undirected_triangle();
        res.add_edge(("c", "d", 1));
        res.add_edge(("d", "e", 1));
        res.add_edge(("e", "f", 1));
        res.add_edge(("g", "h", 1));
        res
    }
    fn get_directed_triangle_with_tail() -> DirectedGraph {
        let mut res = get_directed_triangle();
        res.add_edge(("c", "d", 1));
        res.add_edge(("d", "e", 1));
        res.add_edge(("e", "f", 1));
        res.add_edge(("g", "h", 1));
        res
    }
    fn get_undirected_graph_with_cycle() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("a", "c", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("b", "d", 1));
        res.add_edge(("c", "d", 1));
        res
    }
    fn get_undirected_graph_without_cycle() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("a", "c", 1));
        res.add_edge(("b", "d", 1));
        res.add_edge(("c", "e", 1));
        res
    }
    fn get_directed_graph_with_cycle() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("b", "a", 1));
        res.add_edge(("c", "a", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("c", "d", 1));
        res.add_edge(("d", "b", 1));
        res
    }
    fn get_directed_graph_without_cycle() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("b", "a", 1));
        res.add_edge(("c", "a", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("c", "d", 1));
        res.add_edge(("b", "d", 1));
        res
    }
    macro_rules! test_detect_cycle {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (graph, has_cycle) = $test_case;
                    println!("detect_cycle_dfs: {}", graph.detect_cycle_dfs());
                    println!("detect_cycle_bfs: {}", graph.detect_cycle_bfs());
                    assert_eq!(graph.detect_cycle_dfs(), has_cycle);
                    assert_eq!(graph.detect_cycle_bfs(), has_cycle);
                }
            )*
        };
    }
    test_detect_cycle! {
        undirected_empty: (UndirectedGraph::new(), false),
        directed_empty: (DirectedGraph::new(), false),
        undirected_single_node_with_loop: (get_undirected_single_node_with_loop(), true),
        directed_single_node_with_loop: (get_directed_single_node_with_loop(), true),
        undirected_two_nodes_connected: (get_undirected_two_nodes_connected(), false),
        directed_two_nodes_connected: (get_directed_two_nodes_connected(), true),
        directed_two_nodes: (get_directed_two_nodes(), false),
        undirected_triangle: (get_undirected_triangle(), true),
        undirected_triangle_with_tail: (get_undirected_triangle_with_tail(), true),
        directed_triangle: (get_directed_triangle(), true),
        directed_triangle_with_tail: (get_directed_triangle_with_tail(), true),
        undirected_graph_with_cycle: (get_undirected_graph_with_cycle(), true),
        undirected_graph_without_cycle: (get_undirected_graph_without_cycle(), false),
        directed_graph_with_cycle: (get_directed_graph_with_cycle(), true),
        directed_graph_without_cycle: (get_directed_graph_without_cycle(), false),
    }
}
