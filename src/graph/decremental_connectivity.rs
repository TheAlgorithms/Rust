/// A data-structure that, given a forest, allows dynamic-connectivity queries.
/// Meaning deletion of an edge (u,v) and checking whether two vertecies are still connected.
///
/// # Complexity
/// The preprocessing phase runs in O(n) time, where n is the the number of vertecies in the forest.
/// Deletion runs in O(log n) and checking for connectivity runs in O(1) time.
///
/// # Sources
/// used Wikipedia as reference: <https://en.wikipedia.org/wiki/Dynamic_connectivity>
pub struct DecrementalConnectivity<'a> {
    adjacent: &'a Vec<Vec<usize>>,
    component: Vec<usize>,
    count: usize,
    visited: Vec<usize>,
    dfs_id: usize,
}
impl<'a> DecrementalConnectivity<'a> {
    //expects the parent of a root to be itself
    pub fn new(adjacent: &'a Vec<Vec<usize>>) -> Self {
        let n = adjacent.len();
        let mut tmp = DecrementalConnectivity {
            adjacent,
            component: vec![0; n],
            count: 0,
            visited: vec![0; n],
            dfs_id: 1,
        };
        tmp.component = tmp.calc_component();
        tmp
    }

    pub fn connected(&self, u: usize, v: usize) -> Option<bool> {
        match (self.component.get(u), self.component.get(v)) {
            (Some(a), Some(b)) => Some(a == b),
            _ => None,
        }
    }

    // original adjacency will not be modified by this function
    // expects for the graph to have an edge (u,v)
    pub fn delete(&mut self, u: usize, v: usize) {
        if self.component[u] != self.component[v] {
            return;
        }

        let mut queue: Vec<usize> = Vec::new();

        if self.is_smaller(u, v).expect("invalid indeces") {
            queue.push(u);
            self.dfs_id += 1;
            self.visited[v] = self.dfs_id;
        } else {
            queue.push(v);
            self.dfs_id += 1;
            self.visited[u] = self.dfs_id;
        }
        while !queue.is_empty() {
            let current = queue[0];
            self.dfs_step(&mut queue, self.dfs_id);
            self.component[current] = self.count;
        }
        self.count += 1;
    }

    // Not sure if this is of any use for devs
    pub fn get_component(&self, u: usize) -> usize {
        self.component[u]
    }

    fn calc_component(&mut self) -> Vec<usize> {
        let mut visited: Vec<bool> = vec![false; self.adjacent.len()];
        let mut comp: Vec<usize> = vec![0; self.adjacent.len()];

        for i in 0..self.adjacent.len() {
            if visited[i] {
                continue;
            }
            let mut queue: Vec<usize> = vec![i];
            while let Some(current) = queue.pop() {
                if !visited[current] {
                    queue.append(&mut self.adjacent[current].clone());
                }
                visited[current] = true;
                comp[current] = self.count;
            }
            self.count += 1;
        }
        comp
    }

    fn is_smaller(&mut self, u: usize, v: usize) -> Option<bool> {
        if u >= self.adjacent.len() || u >= self.adjacent.len() {
            return None;
        }

        let mut u_queue: Vec<usize> = vec![u];
        let u_id = self.dfs_id;
        self.visited[v] = u_id;
        self.dfs_id += 1;

        let mut v_queue: Vec<usize> = vec![v];
        let v_id = self.dfs_id;
        self.visited[u] = v_id;
        self.dfs_id += 1;

        // parallel depth first search
        while !u_queue.is_empty() && !v_queue.is_empty() {
            self.dfs_step(&mut u_queue, u_id);
            self.dfs_step(&mut v_queue, v_id);
        }
        Some(u_queue.is_empty())
    }

    fn dfs_step(&mut self, queue: &mut Vec<usize>, dfs_id: usize) {
        let u = queue.pop().unwrap();
        let comp = self.component[u];
        self.visited[u] = dfs_id;
        for v in self.adjacent[u].iter() {
            if self.visited[*v] == dfs_id || self.component[*v] != comp {
                continue;
            }
            queue.push(*v);
        }
    }
}

#[cfg(test)]
mod tests {
    // test forest (remember the assumptoin that roots are adjacent to themselves)
    //              _              _
    //             \ /            \ /
    //              0              7
    //            / | \            |
    //           1  2  3           8
    //         /   / \
    //        4   5   6
    #[test]
    fn construction_test() {
        let adjacent = vec![
            vec![0, 1, 2, 3],
            vec![0, 4],
            vec![0, 5, 6],
            vec![0],
            vec![1],
            vec![2],
            vec![2],
            vec![7, 8],
            vec![7],
        ];
        let dec_con = super::DecrementalConnectivity::new(&adjacent);
        assert_eq!(dec_con.component, vec![0, 0, 0, 0, 0, 0, 0, 1, 1])
    }
    #[test]
    fn query_test() {
        let adjacent = vec![
            vec![0, 1, 2, 3],
            vec![0, 4],
            vec![0, 5, 6],
            vec![0],
            vec![1],
            vec![2],
            vec![2],
            vec![7, 8],
            vec![7],
        ];
        let mut dec_con1 = super::DecrementalConnectivity::new(&adjacent);
        assert_eq!(dec_con1.connected(3, 4), Some(true));
        assert_eq!(dec_con1.connected(5, 0), Some(true));
        assert_eq!(dec_con1.connected(2, 7), Some(false));
        assert_eq!(dec_con1.connected(0, 9), None);
        dec_con1.delete(0, 2);
        assert_eq!(dec_con1.connected(3, 4), Some(true));
        assert_eq!(dec_con1.connected(5, 0), Some(false));
        assert_eq!(dec_con1.connected(5, 6), Some(true));
        assert_eq!(dec_con1.connected(8, 7), Some(true));
        dec_con1.delete(7, 8);
        assert_eq!(dec_con1.connected(8, 7), Some(false));
        dec_con1.delete(7, 8);

        let mut dec_con2 = super::DecrementalConnectivity::new(&adjacent);
        dec_con2.delete(2, 0);
    }
}
