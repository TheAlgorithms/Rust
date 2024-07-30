// Adjacency List
use std::collections::VecDeque;
type Graph = Vec<Vec<usize>>;

pub struct BipartiteMatching {
    pub adj: Graph,
    pub num_vertices_grp1: usize,
    pub num_vertices_grp2: usize,
    // mt1[i] = v is the matching of i in grp1 to v in grp2
    pub mt1: Vec<i32>,
    pub mt2: Vec<i32>,
    pub used: Vec<bool>,
}
impl BipartiteMatching {
    pub fn new(num_vertices_grp1: usize, num_vertices_grp2: usize) -> Self {
        BipartiteMatching {
            adj: vec![vec![]; num_vertices_grp1 + 1],
            num_vertices_grp1,
            num_vertices_grp2,
            mt2: vec![-1; num_vertices_grp2 + 1],
            mt1: vec![-1; num_vertices_grp1 + 1],
            used: vec![false; num_vertices_grp1 + 1],
        }
    }
    #[inline]
    // Add an directed edge u->v in the graph
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }

    fn try_kuhn(&mut self, cur: usize) -> bool {
        if self.used[cur] {
            return false;
        }
        self.used[cur] = true;
        for i in 0..self.adj[cur].len() {
            let to = self.adj[cur][i];
            if self.mt2[to] == -1 || self.try_kuhn(self.mt2[to] as usize) {
                self.mt2[to] = cur as i32;
                return true;
            }
        }
        false
    }
    // Note: It does not modify self.mt1, it only works on self.mt2
    pub fn kuhn(&mut self) {
        self.mt2 = vec![-1; self.num_vertices_grp2 + 1];
        for v in 1..self.num_vertices_grp1 + 1 {
            self.used = vec![false; self.num_vertices_grp1 + 1];
            self.try_kuhn(v);
        }
    }
    pub fn print_matching(&self) {
        for i in 1..self.num_vertices_grp2 + 1 {
            if self.mt2[i] == -1 {
                continue;
            }
            println!("Vertex {} in grp1 matched with {} grp2", self.mt2[i], i)
        }
    }
    fn bfs(&self, dist: &mut [i32]) -> bool {
        let mut q = VecDeque::new();
        for (u, d_i) in dist
            .iter_mut()
            .enumerate()
            .skip(1)
            .take(self.num_vertices_grp1)
        {
            if self.mt1[u] == 0 {
                // u is not matched
                *d_i = 0;
                q.push_back(u);
            } else {
                // else set the vertex distance as infinite because it is matched
                // this will be considered the next time

                *d_i = i32::MAX;
            }
        }
        dist[0] = i32::MAX;
        while !q.is_empty() {
            let u = *q.front().unwrap();
            q.pop_front();
            if dist[u] < dist[0] {
                for i in 0..self.adj[u].len() {
                    let v = self.adj[u][i];
                    if dist[self.mt2[v] as usize] == i32::MAX {
                        dist[self.mt2[v] as usize] = dist[u] + 1;
                        q.push_back(self.mt2[v] as usize);
                    }
                }
            }
        }
        dist[0] != i32::MAX
    }
    fn dfs(&mut self, u: i32, dist: &mut Vec<i32>) -> bool {
        if u == 0 {
            return true;
        }
        for i in 0..self.adj[u as usize].len() {
            let v = self.adj[u as usize][i];
            if dist[self.mt2[v] as usize] == dist[u as usize] + 1 && self.dfs(self.mt2[v], dist) {
                self.mt2[v] = u;
                self.mt1[u as usize] = v as i32;
                return true;
            }
        }
        dist[u as usize] = i32::MAX;
        false
    }
    pub fn hopcroft_karp(&mut self) -> i32 {
        // NOTE: how to use: https://cses.fi/paste/7558dba8d00436a847eab8/
        self.mt2 = vec![0; self.num_vertices_grp2 + 1];
        self.mt1 = vec![0; self.num_vertices_grp1 + 1];
        let mut dist = vec![i32::MAX; self.num_vertices_grp1 + 1];
        let mut res = 0;
        while self.bfs(&mut dist) {
            for u in 1..self.num_vertices_grp1 + 1 {
                if self.mt1[u] == 0 && self.dfs(u as i32, &mut dist) {
                    res += 1;
                }
            }
        }
        // for x in self.mt2 change x to -1 if it is 0
        for x in self.mt2.iter_mut() {
            if *x == 0 {
                *x = -1;
            }
        }
        for x in self.mt1.iter_mut() {
            if *x == 0 {
                *x = -1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_graph_kuhn() {
        let n1 = 6;
        let n2 = 6;
        let mut g = BipartiteMatching::new(n1, n2);
        // vertex 1 in grp1 to vertex 1 in grp 2
        // denote the ith grp2 vertex as n1+i
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        // 2 is not connected to any vertex
        g.add_edge(3, 4);
        g.add_edge(3, 1);
        g.add_edge(4, 3);
        g.add_edge(5, 3);
        g.add_edge(5, 4);
        g.add_edge(6, 6);
        g.kuhn();
        g.print_matching();
        let answer: Vec<i32> = vec![-1, 2, -1, 1, 3, 4, 6];
        for i in 1..g.mt2.len() {
            if g.mt2[i] == -1 {
                // 5 in group2 has no pair
                assert_eq!(i, 5);
                continue;
            }
            // 2 in group1 has no pair
            assert!(g.mt2[i] != 2);
            assert_eq!(i as i32, answer[g.mt2[i] as usize]);
        }
    }
    #[test]
    fn small_graph_hopcroft() {
        let n1 = 6;
        let n2 = 6;
        let mut g = BipartiteMatching::new(n1, n2);
        // vertex 1 in grp1 to vertex 1 in grp 2
        // denote the ith grp2 vertex as n1+i
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        // 2 is not connected to any vertex
        g.add_edge(3, 4);
        g.add_edge(3, 1);
        g.add_edge(4, 3);
        g.add_edge(5, 3);
        g.add_edge(5, 4);
        g.add_edge(6, 6);
        let x = g.hopcroft_karp();
        assert_eq!(x, 5);
        g.print_matching();
        let answer: Vec<i32> = vec![-1, 2, -1, 1, 3, 4, 6];
        for i in 1..g.mt2.len() {
            if g.mt2[i] == -1 {
                // 5 in group2 has no pair
                assert_eq!(i, 5);
                continue;
            }
            // 2 in group1 has no pair
            assert!(g.mt2[i] != 2);
            assert_eq!(i as i32, answer[g.mt2[i] as usize]);
        }
    }
    #[test]
    fn super_small_graph_kuhn() {
        let n1 = 1;
        let n2 = 1;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        g.kuhn();
        g.print_matching();
        assert_eq!(g.mt2[1], 1);
    }
    #[test]
    fn super_small_graph_hopcroft() {
        let n1 = 1;
        let n2 = 1;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        let x = g.hopcroft_karp();
        assert_eq!(x, 1);
        g.print_matching();
        assert_eq!(g.mt2[1], 1);
        assert_eq!(g.mt1[1], 1);
    }

    #[test]
    fn only_one_vertex_graph_kuhn() {
        let n1 = 10;
        let n2 = 10;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        g.add_edge(2, 1);
        g.add_edge(3, 1);
        g.add_edge(4, 1);
        g.add_edge(5, 1);
        g.add_edge(6, 1);
        g.add_edge(7, 1);
        g.add_edge(8, 1);
        g.add_edge(9, 1);
        g.add_edge(10, 1);
        g.kuhn();
        g.print_matching();
        assert_eq!(g.mt2[1], 1);
        for i in 2..g.mt2.len() {
            assert!(g.mt2[i] == -1);
        }
    }
    #[test]
    fn only_one_vertex_graph_hopcroft() {
        let n1 = 10;
        let n2 = 10;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        g.add_edge(2, 1);
        g.add_edge(3, 1);
        g.add_edge(4, 1);
        g.add_edge(5, 1);
        g.add_edge(6, 1);
        g.add_edge(7, 1);
        g.add_edge(8, 1);
        g.add_edge(9, 1);
        g.add_edge(10, 1);
        let x = g.hopcroft_karp();
        assert_eq!(x, 1);
        g.print_matching();
        assert_eq!(g.mt2[1], 1);
        for i in 2..g.mt2.len() {
            assert!(g.mt2[i] == -1);
        }
    }
}
