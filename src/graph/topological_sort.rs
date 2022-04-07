use std::collections::{BTreeMap, VecDeque};

type Graph<V, E> = BTreeMap<V, Vec<(V, E)>>;

/// returns topological sort of the graph using Kahn's algorithm
pub fn topological_sort<V: Ord + Copy, E: Ord>(graph: &Graph<V, E>) -> Vec<V> {
    let mut visited = BTreeMap::new();
    let mut degree = BTreeMap::new();
    for u in graph.keys() {
        degree.insert(*u, 0);
        for (v, _) in graph.get(u).unwrap() {
            let entry = degree.entry(*v).or_insert(0);
            *entry += 1;
        }
    }
    let mut queue = VecDeque::new();
    for (u, d) in degree.iter() {
        if *d == 0 {
            queue.push_back(*u);
            visited.insert(*u, true);
        }
    }
    let mut ret = Vec::new();
    while let Some(u) = queue.pop_front() {
        ret.push(u);
        if let Some(from_u) = graph.get(&u) {
            for (v, _) in from_u {
                *degree.get_mut(v).unwrap() -= 1;
                if *degree.get(v).unwrap() == 0 {
                    queue.push_back(*v);
                    visited.insert(*v, true);
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{topological_sort, Graph};
    fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, from: V, to: V, weight: E) {
        let edges = graph.entry(from).or_insert(Vec::new());
        edges.push((to, weight));
    }

    #[test]
    fn it_works() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 1, 2, 1);
        add_edge(&mut graph, 1, 3, 1);
        add_edge(&mut graph, 2, 3, 1);
        add_edge(&mut graph, 3, 4, 1);
        add_edge(&mut graph, 4, 5, 1);
        add_edge(&mut graph, 5, 6, 1);
        add_edge(&mut graph, 6, 7, 1);

        assert_eq!(topological_sort(&graph), vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
