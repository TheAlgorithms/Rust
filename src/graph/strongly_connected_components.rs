/*
Tarjan's algorithm to find Strongly Connected Components (SCCs):
It runs in O(n + m) (so it is optimal) and as a by-product, it returns the
components in some (reverse) topologically sorted order.

We assume that graph is represented using (compressed) adjacency matrix
and its vertices are numbered from 1 to n. If this is not the case, one
can use `src/graph/graph_enumeration.rs` to convert their graph.
*/

pub struct StronglyConnectedComponents {
    // The number of the SCC the vertex is in, starting from 1
    pub component: Vec<usize>,

    // The discover time of the vertex with minimum discover time reachable
    // from this vertex. The MSB of the numbers are used to save whether the
    // vertex has been visited (but the MSBs are cleared after
    // the algorithm is done)
    pub state: Vec<u64>,

    // The total number of SCCs
    pub num_components: usize,

    // The stack of vertices that DFS has seen (used internally)
    stack: Vec<usize>,
    // Used internally during DFS to know the current discover time
    current_time: usize,
}

// Some functions to help with DRY and code readability
const NOT_DONE: u64 = 1 << 63;

#[inline]
fn set_done(vertex_state: &mut u64) {
    *vertex_state ^= NOT_DONE;
}

#[inline]
fn is_in_stack(vertex_state: u64) -> bool {
    vertex_state != 0 && (vertex_state & NOT_DONE) != 0
}

#[inline]
fn is_unvisited(vertex_state: u64) -> bool {
    vertex_state == NOT_DONE
}

#[inline]
fn get_discover_time(vertex_state: u64) -> u64 {
    vertex_state ^ NOT_DONE
}

impl StronglyConnectedComponents {
    pub fn new(mut num_vertices: usize) -> Self {
        num_vertices += 1; // Vertices are numbered from 1, not 0
        StronglyConnectedComponents {
            component: vec![0; num_vertices],
            state: vec![NOT_DONE; num_vertices],
            num_components: 0,
            stack: vec![],
            current_time: 1,
        }
    }
    fn dfs(&mut self, v: usize, adj: &[Vec<usize>]) -> u64 {
        let mut min_disc = self.current_time as u64;
        // self.state[v] = NOT_DONE + min_disc
        self.state[v] ^= min_disc;
        self.current_time += 1;
        self.stack.push(v);

        for &u in adj[v].iter() {
            if is_unvisited(self.state[u]) {
                min_disc = std::cmp::min(self.dfs(u, adj), min_disc);
            } else if is_in_stack(self.state[u]) {
                min_disc = std::cmp::min(get_discover_time(self.state[u]), min_disc);
            }
        }

        // No vertex with a lower discovery time is reachable from this one
        // So it should be "the head" of a new SCC.
        if min_disc == get_discover_time(self.state[v]) {
            self.num_components += 1;
            loop {
                let u = self.stack.pop().unwrap();
                self.component[u] = self.num_components;
                set_done(&mut self.state[u]);
                if u == v {
                    break;
                }
            }
        }

        min_disc
    }
    pub fn find_components(&mut self, adj: &[Vec<usize>]) {
        self.state[0] = 0;
        for v in 1..adj.len() {
            if is_unvisited(self.state[v]) {
                self.dfs(v, adj);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acyclic() {
        let mut sccs = StronglyConnectedComponents::new(5);
        let adj = vec![vec![], vec![2, 4], vec![3, 4], vec![5], vec![5], vec![]];
        sccs.find_components(&adj);
        assert_eq!(sccs.component, vec![0, 5, 4, 2, 3, 1]);
        assert_eq!(sccs.state, vec![0, 1, 2, 3, 5, 4]);
        assert_eq!(sccs.num_components, 5);
    }

    #[test]
    fn cycle() {
        let mut sccs = StronglyConnectedComponents::new(4);
        let adj = vec![vec![], vec![2], vec![3], vec![4], vec![1]];
        sccs.find_components(&adj);
        assert_eq!(sccs.component, vec![0, 1, 1, 1, 1]);
        assert_eq!(sccs.state, vec![0, 1, 2, 3, 4]);
        assert_eq!(sccs.num_components, 1);
    }

    #[test]
    fn dumbbell() {
        let mut sccs = StronglyConnectedComponents::new(6);
        let adj = vec![
            vec![],
            vec![2],
            vec![3, 4],
            vec![1],
            vec![5],
            vec![6],
            vec![4],
        ];
        sccs.find_components(&adj);
        assert_eq!(sccs.component, vec![0, 2, 2, 2, 1, 1, 1]);
        assert_eq!(sccs.state, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(sccs.num_components, 2);
    }

    #[test]
    fn connected_dumbbell() {
        let mut sccs = StronglyConnectedComponents::new(6);
        let adj = vec![
            vec![],
            vec![2],
            vec![3, 4],
            vec![1],
            vec![5, 1],
            vec![6],
            vec![4],
        ];
        sccs.find_components(&adj);
        assert_eq!(sccs.component, vec![0, 1, 1, 1, 1, 1, 1]);
        assert_eq!(sccs.state, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(sccs.num_components, 1);
    }
}
