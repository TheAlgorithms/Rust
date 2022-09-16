// Adjacency List
type Graph = Vec<Vec<usize>>;

pub struct BipartiteMatching {
    pub adj: Graph,
    pub num_vertices_grp1: usize,
    pub num_vertices_grp2: usize,
    // mt[i] = v is the matching of i in grp1 to v in grp2
    pub mt: Vec<i32>,
    pub used: Vec<bool>,
}
impl BipartiteMatching {
    pub fn new(num_vertices_grp1: usize, num_vertices_grp2: usize) -> Self {
        BipartiteMatching {
            adj: vec![vec![]; num_vertices_grp1 + 1],
            num_vertices_grp1,
            num_vertices_grp2,
            mt: vec![-1; num_vertices_grp2 + 1],
            used: vec![false; num_vertices_grp1 + 1],
        }
    }
    #[inline]
    // Add an undirected edge u-v in the graph
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        // self.adj[v].push(u);
    }

    fn try_kuhn(&mut self, cur: usize) -> bool {
        if self.used[cur] {
            return false;
        }
        self.used[cur] = true;
        for i in 0..self.adj[cur].len() {
            let to = self.adj[cur][i];
            if self.mt[to] == -1 || self.try_kuhn(self.mt[to] as usize) {
                self.mt[to] = cur as i32;
                return true;
            }
        }
        return false;
    }
    pub fn khun(&mut self) {
        self.mt = vec![-1; self.num_vertices_grp2 + 1];
        for v in 1..self.num_vertices_grp1 + 1 {
            self.used = vec![false; self.num_vertices_grp1 + 1];
            self.try_kuhn(v);
        }
    }
    pub fn print_matching(&self) {
        for i in 1..self.num_vertices_grp2 + 1 {
            if self.mt[i] == -1 {
                continue;
            }
            println!("Vertex {} in grp1 matched with {} grp2", self.mt[i], i)
        }
    }
}
#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;
    #[test]
    fn small_graph() {
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
        g.khun();
        g.print_matching();
        let answer: Vec<i32> = vec![-1, 2, -1, 1, 3, 4, 6];
        for i in 1..g.mt.len() {
            if g.mt[i] == -1 {
                // 5 in group2 has no pair
                assert_eq!(i, 5);
                continue;
            }
            // 2 in group1 has no pair
            assert!(g.mt[i] != 2);
            assert_eq!(i as i32, answer[g.mt[i] as usize]);
        }
    }
    #[test]
    fn super_small_graph() {
        let n1 = 1;
        let n2 = 1;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        g.khun();
        g.print_matching();
        assert_eq!(g.mt[1], 1);
    }
    #[test]
    fn only_one_vertex_graph() {
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
        g.khun();
        g.print_matching();
        assert_eq!(g.mt[1], 1);
        for i in 2..g.mt.len() {
            assert!(g.mt[i] == -1);
        }
    }
}
