use std::{
    collections::{BTreeMap, BinaryHeap},
    ops::Add,
};

use num_traits::Zero;

type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Candidate<V, E> {
    estimated_weight: E,
    real_weight: E,
    state: V,
}

impl<V: Ord + Copy, E: Ord + Copy> PartialOrd for Candidate<V, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Note the inverted order; we want nodes with lesser weight to have
        // higher priority
        Some(self.cmp(other))
    }
}

impl<V: Ord + Copy, E: Ord + Copy> Ord for Candidate<V, E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Note the inverted order; we want nodes with lesser weight to have
        // higher priority
        other.estimated_weight.cmp(&self.estimated_weight)
    }
}

pub fn astar<V: Ord + Copy, E: Ord + Copy + Add<Output = E> + Zero>(
    graph: &Graph<V, E>,
    start: V,
    target: V,
    heuristic: impl Fn(V) -> E,
) -> Option<(E, Vec<V>)> {
    // traversal front
    let mut queue = BinaryHeap::new();
    // maps each node to its predecessor in the final path
    let mut previous = BTreeMap::new();
    // weights[v] is the accumulated weight from start to v
    let mut weights = BTreeMap::new();
    // initialize traversal
    weights.insert(start, E::zero());
    queue.push(Candidate {
        estimated_weight: heuristic(start),
        real_weight: E::zero(),
        state: start,
    });
    while let Some(Candidate {
        estimated_weight: _,
        real_weight,
        state: current,
    }) = queue.pop()
    {
        if current == target {
            break;
        }
        for (&next, &weight) in &graph[&current] {
            let real_weight = real_weight + weight;
            if weights
                .get(&next)
                .map_or(true, |&weight| real_weight < weight)
            {
                // current allows us to reach next with lower weight (or at all)
                // add next to the front
                let estimated_weight = real_weight + heuristic(next);
                weights.insert(next, real_weight);
                queue.push(Candidate {
                    estimated_weight,
                    real_weight,
                    state: next,
                });
                previous.insert(next, current);
            }
        }
    }
    let weight = if let Some(&weight) = weights.get(&target) {
        weight
    } else {
        // we did not reach target from start
        return None;
    };
    // build path in reverse
    let mut current = target;
    let mut path = vec![current];
    while current != start {
        let prev = previous
            .get(&current)
            .copied()
            .expect("We reached the target, but are unable to reconsistute the path");
        current = prev;
        path.push(current);
    }
    path.reverse();
    Some((weight, path))
}

#[cfg(test)]
mod tests {
    use super::{astar, Graph};
    use num_traits::Zero;
    use std::collections::BTreeMap;

    // the null heuristic make A* equivalent to Dijkstra
    fn null_heuristic<V, E: Zero>(_v: V) -> E {
        E::zero()
    }

    fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, v1: V, v2: V, c: E) {
        graph.entry(v1).or_default().insert(v2, c);
        graph.entry(v2).or_default();
    }

    #[test]
    fn single_vertex() {
        let mut graph: Graph<usize, usize> = BTreeMap::new();
        graph.insert(0, BTreeMap::new());

        assert_eq!(astar(&graph, 0, 0, null_heuristic), Some((0, vec![0])));
        assert_eq!(astar(&graph, 0, 1, null_heuristic), None);
    }

    #[test]
    fn single_edge() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 2);

        assert_eq!(astar(&graph, 0, 1, null_heuristic), Some((2, vec![0, 1])));
        assert_eq!(astar(&graph, 1, 0, null_heuristic), None);
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

        // from a
        assert_eq!(
            astar(&graph, 'a', 'a', null_heuristic),
            Some((0, vec!['a']))
        );
        assert_eq!(
            astar(&graph, 'a', 'b', null_heuristic),
            Some((32, vec!['a', 'c', 'b']))
        );
        assert_eq!(
            astar(&graph, 'a', 'c', null_heuristic),
            Some((12, vec!['a', 'c']))
        );
        assert_eq!(
            astar(&graph, 'a', 'd', null_heuristic),
            Some((12 + 32, vec!['a', 'c', 'd']))
        );
        assert_eq!(astar(&graph, 'a', 'e', null_heuristic), None);

        // from b
        assert_eq!(
            astar(&graph, 'b', 'a', null_heuristic),
            Some((10, vec!['b', 'a']))
        );
        assert_eq!(
            astar(&graph, 'b', 'b', null_heuristic),
            Some((0, vec!['b']))
        );
        assert_eq!(
            astar(&graph, 'b', 'c', null_heuristic),
            Some((10 + 12, vec!['b', 'a', 'c']))
        );
        assert_eq!(
            astar(&graph, 'b', 'd', null_heuristic),
            Some((10 + 12 + 32, vec!['b', 'a', 'c', 'd']))
        );
        assert_eq!(astar(&graph, 'b', 'e', null_heuristic), None);

        // from c
        assert_eq!(
            astar(&graph, 'c', 'a', null_heuristic),
            Some((20 + 10, vec!['c', 'b', 'a']))
        );
        assert_eq!(
            astar(&graph, 'c', 'b', null_heuristic),
            Some((20, vec!['c', 'b']))
        );
        assert_eq!(
            astar(&graph, 'c', 'c', null_heuristic),
            Some((0, vec!['c']))
        );
        assert_eq!(
            astar(&graph, 'c', 'd', null_heuristic),
            Some((32, vec!['c', 'd']))
        );
        assert_eq!(astar(&graph, 'c', 'e', null_heuristic), None);

        // from d
        assert_eq!(astar(&graph, 'd', 'a', null_heuristic), None);
        assert_eq!(astar(&graph, 'd', 'b', null_heuristic), None);
        assert_eq!(astar(&graph, 'd', 'c', null_heuristic), None);
        assert_eq!(
            astar(&graph, 'd', 'd', null_heuristic),
            Some((0, vec!['d']))
        );
        assert_eq!(astar(&graph, 'd', 'e', null_heuristic), None);

        // from e
        assert_eq!(
            astar(&graph, 'e', 'a', null_heuristic),
            Some((7, vec!['e', 'a']))
        );
        assert_eq!(
            astar(&graph, 'e', 'b', null_heuristic),
            Some((7 + 12 + 20, vec!['e', 'a', 'c', 'b']))
        );
        assert_eq!(
            astar(&graph, 'e', 'c', null_heuristic),
            Some((7 + 12, vec!['e', 'a', 'c']))
        );
        assert_eq!(
            astar(&graph, 'e', 'd', null_heuristic),
            Some((7 + 12 + 32, vec!['e', 'a', 'c', 'd']))
        );
        assert_eq!(
            astar(&graph, 'e', 'e', null_heuristic),
            Some((0, vec!['e']))
        );
    }

    #[test]
    fn test_heuristic() {
        // make a grid
        let mut graph = BTreeMap::new();
        let rows = 100;
        let cols = 100;
        for row in 0..rows {
            for col in 0..cols {
                add_edge(&mut graph, (row, col), (row + 1, col), 1);
                add_edge(&mut graph, (row, col), (row, col + 1), 1);
                add_edge(&mut graph, (row, col), (row + 1, col + 1), 1);
                add_edge(&mut graph, (row + 1, col), (row, col), 1);
                add_edge(&mut graph, (row + 1, col + 1), (row, col), 1);
            }
        }

        // Dijkstra would explore most of the 101 × 101 nodes
        // the heuristic should allow exploring only about 200 nodes
        let now = std::time::Instant::now();
        let res = astar(&graph, (0, 0), (100, 90), |(i, j)| 100 - i + 90 - j);
        assert!(now.elapsed() < std::time::Duration::from_millis(10));

        let (weight, path) = res.unwrap();
        assert_eq!(weight, 100);
        assert_eq!(path.len(), 101);
    }
}
