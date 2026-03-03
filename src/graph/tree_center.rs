//! Finds the center of a weighted tree using two depth-first searches (DFS).
//!
//! The algorithm works as follows:
//!
//! 1. Perform a first DFS to compute, for each node, the maximum distance
//!    to any descendant in its subtree (i.e., the deepest node below it).
//!
//! 2. Perform a second DFS to compute, for each node, the maximum distance
//!    to nodes outside its subtree (i.e., through its parent and other branches).
//!
//! For each vertex, the farthest node in the tree is either:
//! - a descendant in its own subtree, or
//! - a node reached by going up to its parent and then down another branch.
//!
//! The eccentricity of each node is the maximum of these two values.
//! The center of the tree is the node that minimizes this eccentricity.
//!
//! This implementation works for weighted trees.

use std::collections::HashMap;

use crate::data_structures::{graph::Graph, UndirectedGraph};

type Table<V> = HashMap<V, i64>;

const INF: i64 = 1_000_000_000_000_000_000;

fn depth_first_search_down<'a>(
    tree: &'a UndirectedGraph,
    dist: &mut Table<&'a String>,
    max_down: &mut Table<&'a String>,
    u: &'a String,
    parent: Option<&'a String>,
) {
    let dist_from_root = *dist.get(u).unwrap();

    let mut max_dist_down = dist_from_root;

    for (v, weight) in tree.adjacency_table().get(u).unwrap() {
        if parent == Some(v) {
            continue;
        }

        dist.insert(v, dist_from_root + *weight as i64);

        depth_first_search_down(tree, dist, max_down, v, Some(u));

        max_dist_down = max_dist_down.max(*max_down.get(v).unwrap());
    }

    max_down.insert(u, max_dist_down);
}

fn depth_first_search_up<'a>(
    tree: &'a UndirectedGraph,
    dist: &mut Table<&'a String>,
    max_dist: &mut Table<&'a String>,
    max_down: &mut Table<&'a String>,
    u: &'a String,
    parent: Option<&'a String>,
    mut max_up: i64,
) {
    let mut first_max_down = -INF;
    let mut second_max_down = -INF;

    for (v, _) in tree.adjacency_table().get(u).unwrap() {
        if parent == Some(v) {
            continue;
        }

        let dist_max_down = *max_down.get(v).unwrap();

        if first_max_down < dist_max_down {
            second_max_down = first_max_down;
            first_max_down = dist_max_down;
        } else {
            second_max_down = second_max_down.max(dist_max_down);
        }
    }

    let dist_from_root = *dist.get(u).unwrap();

    max_up = max_up.max(0);

    max_dist.insert(u, max_up.max(*max_down.get(u).unwrap() - dist_from_root));

    for (v, weight) in tree.adjacency_table().get(u).unwrap() {
        if parent == Some(v) {
            continue;
        }

        let dist_max_down = *max_down.get(v).unwrap();

        let mut max_dist_up = if first_max_down == dist_max_down {
            second_max_down
        } else {
            first_max_down
        };

        max_dist_up = max_up.max(max_dist_up - dist_from_root) + *weight as i64;

        depth_first_search_up(tree, dist, max_dist, max_down, v, Some(u), max_dist_up);
    }
}

pub fn tree_center(tree: &UndirectedGraph) -> Option<Vec<String>> {
    let node = tree.adjacency_table().keys().last();

    if Option::is_none(&node) {
        return None;
    }

    let mut dist = Table::new();

    let mut max_dist = Table::new();

    let mut max_down = Table::new();

    let root = node.unwrap();

    dist.insert(root, 0);

    depth_first_search_down(tree, &mut dist, &mut max_down, root, None);

    depth_first_search_up(
        tree,
        &mut dist,
        &mut max_dist,
        &mut max_down,
        root,
        None,
        -INF,
    );

    let min_dist = max_dist.iter().map(|v| *v.1).min().unwrap();

    let center = max_dist
        .iter()
        .filter(|v| *v.1 == min_dist)
        .map(|v| (*v.0).clone())
        .collect::<Vec<_>>();

    Some(center)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let tree = UndirectedGraph::new();

        let center = tree_center(&tree);

        assert_eq!(center, None);
    }

    #[test]
    fn test_trivial_graph() {
        let mut tree = UndirectedGraph::new();
        let expected = vec!["0".to_string()];

        tree.add_node("0");

        let center = tree_center(&tree).unwrap();

        assert_eq!(center, expected);
    }

    #[test]
    fn test_edge() {
        let mut tree = UndirectedGraph::new();
        let expected = vec!["0".to_string(), "1".to_string()];

        tree.add_edge(("0", "1", 1));

        let mut center = tree_center(&tree).unwrap();

        center.sort();

        assert_eq!(center, expected);
    }

    #[test]
    fn test_simple_path() {
        let mut tree = UndirectedGraph::new();
        let expected = vec!["2".to_string(), "3".to_string()];

        tree.add_edge(("0", "1", 1));
        tree.add_edge(("1", "2", 1));
        tree.add_edge(("2", "3", 1));
        tree.add_edge(("3", "4", 1));
        tree.add_edge(("4", "5", 1));

        let mut center = tree_center(&tree).unwrap();

        center.sort();

        assert_eq!(center, expected);
    }

    #[test]
    fn test_star_tree() {
        let mut tree = UndirectedGraph::new();
        let expected = vec!["0".to_string()];

        tree.add_edge(("0", "1", 1));
        tree.add_edge(("0", "2", 1));
        tree.add_edge(("0", "3", 1));
        tree.add_edge(("0", "4", 1));

        let center = tree_center(&tree).unwrap();

        assert_eq!(center, expected);
    }

    #[test]
    fn test_double_star_tree() {
        let mut tree = UndirectedGraph::new();
        let expected = vec!["0".to_string(), "1".to_string()];

        tree.add_edge(("0", "2", 1));
        tree.add_edge(("0", "3", 1));
        tree.add_edge(("0", "4", 1));
        tree.add_edge(("1", "5", 1));
        tree.add_edge(("1", "6", 1));
        tree.add_edge(("1", "7", 1));
        tree.add_edge(("0", "1", 1));

        let mut center = tree_center(&tree).unwrap();

        center.sort();

        assert_eq!(center, expected);
    }

    #[test]
    fn test_simple_path_10_vertices_tree() {
        let mut tree = UndirectedGraph::new();
        let expected = ["10".to_string(), "9".to_string()];

        tree.add_edge(("4", "1", 1));
        tree.add_edge(("6", "5", 1));
        tree.add_edge(("7", "2", 1));
        tree.add_edge(("6", "3", 1));
        tree.add_edge(("1", "7", 1));
        tree.add_edge(("2", "10", 1));
        tree.add_edge(("10", "9", 1));
        tree.add_edge(("3", "8", 1));
        tree.add_edge(("8", "9", 1));

        let mut center = tree_center(&tree).unwrap();

        center.sort();

        assert_eq!(center, expected);
    }

    #[test]
    fn test_simple_weighted_path() {
        let mut tree = UndirectedGraph::new();
        let expected = vec!["2".to_string()];

        tree.add_edge(("4", "2", 10));
        tree.add_edge(("2", "3", 5));
        tree.add_edge(("3", "1", 5));

        let center = tree_center(&tree).unwrap();

        assert_eq!(center, expected);
    }

    #[test]
    fn test_double_star_weighted_tree() {
        let mut tree = UndirectedGraph::new();
        let expected = vec!["1".to_string()];

        tree.add_edge(("1", "2", 4));
        tree.add_edge(("1", "3", 4));
        tree.add_edge(("1", "4", 1));
        tree.add_edge(("4", "5", 1));

        let center = tree_center(&tree).unwrap();

        assert_eq!(center, expected);
    }

    #[test]
    fn test_star_weighted_tree() {
        let mut tree = UndirectedGraph::new();
        let expected = vec!["1".to_string(), "2".to_string()];

        tree.add_edge(("1", "2", 0));
        tree.add_edge(("1", "3", 2));
        tree.add_edge(("1", "4", 2));

        let mut center = tree_center(&tree).unwrap();

        center.sort();

        assert_eq!(center, expected);
    }
}
