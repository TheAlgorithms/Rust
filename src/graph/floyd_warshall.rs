use num_traits::Zero;
use std::collections::BTreeMap;
use std::ops::Add;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

/// Performs the Floyd-Warshall algorithm on the input graph.\
/// The graph is a weighted, directed graph with no negative cycles.
///
/// Returns a map storing the distance from each node to all the others.\
/// i.e. For each vertex `u`, `map[u][v] == Some(distance)` means
/// distance is the sum of the weights of the edges on the shortest path
/// from `u` to `v`.
///
/// For a key `v`, if `map[v].len() == 0`, then `v` cannot reach any other vertex, but is in the graph
/// (island node, or sink in the case of a directed graph)
pub fn floyd_warshall<V: Ord + Copy, E: Ord + Copy + Add<Output = E> + num_traits::Zero>(
    graph: &Graph<V, E>,
) -> BTreeMap<V, BTreeMap<V, E>> {
    let mut map: BTreeMap<V, BTreeMap<V, E>> = BTreeMap::new();
    for (u, edges) in graph.iter() {
        if !map.contains_key(u) {
            map.insert(*u, BTreeMap::new());
        }
        map.entry(*u).or_default().insert(*u, Zero::zero());
        for (v, weight) in edges.iter() {
            if !map.contains_key(v) {
                map.insert(*v, BTreeMap::new());
            }
            map.entry(*v).or_default().insert(*v, Zero::zero());
            map.entry(*u).and_modify(|mp| {
                mp.insert(*v, *weight);
            });
        }
    }
    let keys = map.keys().copied().collect::<Vec<_>>();
    for &k in &keys {
        for &i in &keys {
            if !map[&i].contains_key(&k) {
                continue;
            }
            for &j in &keys {
                if i == j {
                    continue;
                }
                if !map[&k].contains_key(&j) {
                    continue;
                }
                let entry_i_j = map[&i].get(&j);
                let entry_i_k = map[&i][&k];
                let entry_k_j = map[&k][&j];
                match entry_i_j {
                    Some(&e) => {
                        if e > entry_i_k + entry_k_j {
                            map.entry(i).or_default().insert(j, entry_i_k + entry_k_j);
                        }
                    }
                    None => {
                        map.entry(i).or_default().insert(j, entry_i_k + entry_k_j);
                    }
                };
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::{floyd_warshall, Graph};
    use std::collections::BTreeMap;

    fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_default().insert(v2, c);
    }

    fn bi_add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        add_edge(graph, v1, v2, c);
        add_edge(graph, v2, v1, c);
    }

    #[test]
    fn single_vertex() {
        let mut graph: Graph<usize, usize> = BTreeMap::new();
        graph.insert(0, BTreeMap::new());

        let mut dists = BTreeMap::new();
        dists.insert(0, BTreeMap::new());
        dists.get_mut(&0).unwrap().insert(0, 0);
        assert_eq!(floyd_warshall(&graph), dists);
    }

    #[test]
    fn single_edge() {
        let mut graph = BTreeMap::new();
        bi_add_edge(&mut graph, 0, 1, 2);
        bi_add_edge(&mut graph, 1, 2, 3);

        let mut dists_0 = BTreeMap::new();
        dists_0.insert(0, BTreeMap::new());
        dists_0.insert(1, BTreeMap::new());
        dists_0.insert(2, BTreeMap::new());
        dists_0.get_mut(&0).unwrap().insert(0, 0);
        dists_0.get_mut(&1).unwrap().insert(1, 0);
        dists_0.get_mut(&2).unwrap().insert(2, 0);
        dists_0.get_mut(&1).unwrap().insert(0, 2);
        dists_0.get_mut(&0).unwrap().insert(1, 2);
        dists_0.get_mut(&1).unwrap().insert(2, 3);
        dists_0.get_mut(&2).unwrap().insert(1, 3);
        dists_0.get_mut(&2).unwrap().insert(0, 5);
        dists_0.get_mut(&0).unwrap().insert(2, 5);

        assert_eq!(floyd_warshall(&graph), dists_0);
    }

    #[test]
    fn graph_1() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 'a', 'c', 12);
        add_edge(&mut graph, 'a', 'd', 60);
        add_edge(&mut graph, 'b', 'a', 10);
        add_edge(&mut graph, 'c', 'b', 20);
        add_edge(&mut graph, 'c', 'd', 32);
        add_edge(&mut graph, 'e', 'a', 7);

        let mut dists_a = BTreeMap::new();
        dists_a.insert('d', BTreeMap::new());

        dists_a.entry('a').or_insert(BTreeMap::new()).insert('a', 0);
        dists_a.entry('b').or_insert(BTreeMap::new()).insert('b', 0);
        dists_a.entry('c').or_insert(BTreeMap::new()).insert('c', 0);
        dists_a.entry('d').or_insert(BTreeMap::new()).insert('d', 0);
        dists_a.entry('e').or_insert(BTreeMap::new()).insert('e', 0);
        dists_a
            .entry('a')
            .or_insert(BTreeMap::new())
            .insert('c', 12);
        dists_a
            .entry('c')
            .or_insert(BTreeMap::new())
            .insert('a', 30);
        dists_a
            .entry('c')
            .or_insert(BTreeMap::new())
            .insert('b', 20);
        dists_a
            .entry('c')
            .or_insert(BTreeMap::new())
            .insert('d', 32);
        dists_a.entry('e').or_insert(BTreeMap::new()).insert('a', 7);
        dists_a
            .entry('b')
            .or_insert(BTreeMap::new())
            .insert('a', 10);
        dists_a
            .entry('a')
            .or_insert(BTreeMap::new())
            .insert('d', 44);
        dists_a
            .entry('a')
            .or_insert(BTreeMap::new())
            .insert('b', 32);
        dists_a
            .entry('a')
            .or_insert(BTreeMap::new())
            .insert('b', 32);
        dists_a
            .entry('b')
            .or_insert(BTreeMap::new())
            .insert('c', 22);

        dists_a
            .entry('b')
            .or_insert(BTreeMap::new())
            .insert('d', 54);
        dists_a
            .entry('e')
            .or_insert(BTreeMap::new())
            .insert('c', 19);
        dists_a
            .entry('e')
            .or_insert(BTreeMap::new())
            .insert('d', 51);
        dists_a
            .entry('e')
            .or_insert(BTreeMap::new())
            .insert('b', 39);

        assert_eq!(floyd_warshall(&graph), dists_a);
    }
}
