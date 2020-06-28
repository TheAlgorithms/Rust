use graphs::container::{Graph, Node};

use std::collections::VecDeque;
use std::fmt;

/// Do a deoth first traversal of the container::Graph structure from the
/// given `root` node. The function accepts a closure that accepts a ref
/// to the Node and can read from it and returns nothing. It will run
/// exactly once per node. The functions returns an ordered Vec<T> of the
/// Node.val.
pub fn depth_first_search<T, F>(graph: &Graph<T>, root: usize, f: F) -> Vec<T>
where
    T: fmt::Display + Clone,
    F: Fn(&Node<T>) -> (),
{
    let mut st = VecDeque::<usize>::new();
    let mut ret = Vec::new();
    let mut vis = Vec::<bool>::new();
    vis.resize(graph.g.len(), false);

    st.push_back(root);
    while st.len() > 0 {
        let r = st.pop_back().unwrap();
        f(&graph.g[r]);
        ret.push(graph.g[r].val.clone());
        vis[r] = true;

        for i in &graph.g[r].edges {
            if !vis[*i] {
                st.push_back(*i);
            }
        }
    }

    ret
}

/// Do a breadth first traversal of the container::Graph structure from the
/// given `root` node. The function accepts a closure that accepts a ref
/// to the Node and can read from it and returns nothing. It will run
/// exactly once per node. The functions returns an ordered Vec<T> of the
/// Node.val.
pub fn breadth_first_search<T, F>(graph: &Graph<T>, root: usize, f: F) -> Vec<T>
where
    T: fmt::Display + Clone,
    F: Fn(&Node<T>) -> (),
{
    let mut q = VecDeque::<usize>::new();
    let mut ret = Vec::new();
    let mut vis = Vec::<bool>::new();
    vis.resize(graph.g.len(), false);
    q.push_back(root);
    while q.len() > 0 {
        let r = q.pop_front().unwrap();
        f(&graph.g[r]);

        ret.push(graph.g[r].val.clone());
        vis[r] = true;

        for i in &graph.g[r].edges {
            if vis[*i] == false {
                q.push_back(*i);
            }
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn graphs_dfs_1() {
        let mut g = Graph::<i32>::new();
        let n1 = Node::<i32>::new(5);
        let n2 = Node::<i32>::new(7);
        let n3 = Node::<i32>::new(3);

        g.push(n1);
        g.push(n2);
        g.push(n3);

        g.add_edge(0, 1);
        g.add_edge(1, 0);
        g.add_edge(1, 2);

        let v = depth_first_search(&g, 0, |x: &Node<i32>| println!("{}", x.val));
        assert_eq!(v, vec![5, 7, 3])
    }

    #[test]
    fn graphs_dfs_2() {
        let mut g = Graph::<i32>::new();
        let n1 = Node::<i32>::new(1);
        let n2 = Node::<i32>::new(2);
        let n3 = Node::<i32>::new(3);
        let n4 = Node::<i32>::new(4);

        g.push(n1);
        g.push(n2);
        g.push(n3);
        g.push(n4);

        g.add_edge(0, 1);
        g.add_edge(1, 0);
        g.add_edge(1, 3);
        g.add_edge(1, 2);

        // 0(1) -> [1]
        // 1(2) -> [0, 3, 2]
        // 2(3) -> []
        // 3(4) -> []
        let v = depth_first_search(&g, 1, |x: &Node<i32>| println!("{}", x.val));
        assert_eq!(v, vec![2, 3, 4, 1])
    }

    #[test]
    fn graphs_bfs_1() {
        let mut g = Graph::<i32>::new();
        let n1 = Node::<i32>::new(5);
        let n2 = Node::<i32>::new(7);
        let n3 = Node::<i32>::new(3);

        g.push(n1);
        g.push(n2);
        g.push(n3);

        g.add_edge(0, 1);
        g.add_edge(1, 0);
        g.add_edge(1, 2);

        let v = breadth_first_search(&g, 0, |x: &Node<i32>| println!("{}", x.val));
        assert_eq!(v, vec![5, 7, 3])
    }

    #[test]
    fn graphs_bfs_2() {
        let mut g = Graph::<i32>::new();
        let n1 = Node::<i32>::new(1);
        let n2 = Node::<i32>::new(2);
        let n3 = Node::<i32>::new(3);
        let n4 = Node::<i32>::new(4);

        g.push(n1);
        g.push(n2);
        g.push(n3);
        g.push(n4);

        g.add_edge(0, 1);
        g.add_edge(1, 0);
        g.add_edge(1, 3);
        g.add_edge(1, 2);

        // 0(1) -> [1]
        // 1(2) -> [0, 3, 2]
        // 2(3) -> []
        // 3(4) -> []
        let v = breadth_first_search(&g, 1, |x: &Node<i32>| println!("{}", x.val));
        assert_eq!(v, vec![2, 1, 4, 3])
    }
}
