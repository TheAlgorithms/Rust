use std::collections::VecDeque;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

// We assume that graph vertices are numbered from 1 to n.

/// Adjacency matrix
type Graph = Vec<Vec<usize>>;

/// We assume that T::default() gives "zero" flow and T supports negative values
pub struct FlowEdge<T> {
    pub sink: usize,
    pub capacity: T,
    pub flow: T,
}

pub struct FlowResultEdge<T> {
    pub source: usize,
    pub sink: usize,
    pub flow: T,
}

impl<T: Clone + Copy + Add + AddAssign + Sub<Output = T> + SubAssign + Ord + Neg + Default>
    FlowEdge<T>
{
    pub fn new(sink: usize, capacity: T) -> Self {
        FlowEdge {
            sink,
            capacity,
            flow: T::default(),
        }
    }
}

pub struct DinicMaxFlow<T> {
    /// BFS Level of each vertex. starts from 1
    level: Vec<usize>,

    /// The index of the last visited edge connected to each vertex
    pub last_edge: Vec<usize>,

    /// Holds wether the solution has already been calculated
    network_solved: bool,

    pub source: usize,
    pub sink: usize,

    /// Number of edges added to the residual network
    pub num_edges: usize,
    pub num_vertices: usize,

    pub adj: Graph,

    /// The list of flow edges
    pub edges: Vec<FlowEdge<T>>,
}

impl<T: Clone + Copy + Add + AddAssign + Sub<Output = T> + SubAssign + Neg + Ord + Default>
    DinicMaxFlow<T>
{
    pub fn new(source: usize, sink: usize, num_vertices: usize) -> Self {
        DinicMaxFlow {
            level: vec![0; num_vertices + 1],
            last_edge: vec![0; num_vertices + 1],
            network_solved: false,
            source,
            sink,
            num_edges: 0,
            num_vertices,
            adj: vec![vec![]; num_vertices + 1],
            edges: vec![],
        }
    }
    #[inline]
    pub fn add_edge(&mut self, source: usize, sink: usize, capacity: T) {
        self.edges.push(FlowEdge::new(sink, capacity));
        // Add the reverse edge with zero capacity
        self.edges.push(FlowEdge::new(source, T::default()));
        // We inserted the m'th edge from source to sink
        self.adj[source].push(self.num_edges);
        self.adj[sink].push(self.num_edges + 1);
        self.num_edges += 2;
    }

    fn bfs(&mut self) -> bool {
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(self.source);

        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            for &e in self.adj[v].iter() {
                if self.edges[e].capacity <= self.edges[e].flow {
                    continue;
                }
                let u = self.edges[e].sink;
                if self.level[u] != 0 {
                    continue;
                }
                self.level[u] = self.level[v] + 1;
                q.push_back(u);
            }
        }

        self.level[self.sink] != 0
    }

    fn dfs(&mut self, v: usize, pushed: T) -> T {
        // We have pushed nothing, or we are at the sink
        if v == self.sink {
            return pushed;
        }
        for e_pos in self.last_edge[v]..self.adj[v].len() {
            let e = self.adj[v][e_pos];
            let u = self.edges[e].sink;
            if (self.level[v] + 1) != self.level[u] || self.edges[e].capacity <= self.edges[e].flow
            {
                continue;
            }
            let down_flow = self.dfs(
                u,
                std::cmp::min(pushed, self.edges[e].capacity - self.edges[e].flow),
            );
            if down_flow == T::default() {
                continue;
            }
            self.last_edge[v] = e_pos;
            self.edges[e].flow += down_flow;
            self.edges[e ^ 1].flow -= down_flow;
            return down_flow;
        }
        self.last_edge[v] = self.adj[v].len();
        T::default()
    }

    pub fn find_maxflow(&mut self, infinite_flow: T) -> T {
        self.network_solved = true;
        let mut total_flow: T = T::default();
        loop {
            self.level.fill(0);
            self.level[self.source] = 1;
            // There is no longer a path from source to sink in the residual
            // network
            if !self.bfs() {
                break;
            }
            self.last_edge.fill(0);
            let mut next_flow = self.dfs(self.source, infinite_flow);
            while next_flow != T::default() {
                total_flow += next_flow;
                next_flow = self.dfs(self.source, infinite_flow);
            }
        }
        total_flow
    }

    pub fn get_flow_edges(&mut self, infinite_flow: T) -> Vec<FlowResultEdge<T>> {
        if !self.network_solved {
            self.find_maxflow(infinite_flow);
        }
        let mut result = Vec::new();
        for v in 1..self.adj.len() {
            for &e_ind in self.adj[v].iter() {
                let e = &self.edges[e_ind];
                // Make sure that reverse edges from residual network are not
                // included
                if e.flow > T::default() {
                    result.push(FlowResultEdge {
                        source: v,
                        sink: e.sink,
                        flow: e.flow,
                    });
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_graph() {
        let mut flow: DinicMaxFlow<i32> = DinicMaxFlow::new(1, 6, 6);
        flow.add_edge(1, 2, 16);
        flow.add_edge(1, 4, 13);
        flow.add_edge(2, 3, 12);
        flow.add_edge(3, 4, 9);
        flow.add_edge(3, 6, 20);
        flow.add_edge(4, 2, 4);
        flow.add_edge(4, 5, 14);
        flow.add_edge(5, 3, 7);
        flow.add_edge(5, 6, 4);

        let max_flow = flow.find_maxflow(i32::MAX);
        assert_eq!(max_flow, 23);

        let mut sm_out = [0; 7];
        let mut sm_in = [0; 7];

        let flow_edges = flow.get_flow_edges(i32::MAX);
        for e in flow_edges {
            sm_out[e.source] += e.flow;
            sm_in[e.sink] += e.flow;
        }
        for i in 2..=5 {
            assert_eq!(sm_in[i], sm_out[i]);
        }
        assert_eq!(sm_in[1], 0);
        assert_eq!(sm_out[1], max_flow);
        assert_eq!(sm_in[6], max_flow);
        assert_eq!(sm_out[6], 0);
    }
}
