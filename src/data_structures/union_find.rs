/// UnionFind data structure
pub struct UnionFind {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    /// Creates a new UnionFind data structure with n elements
    pub fn new(n: usize) -> Self {
        let mut id = vec![0; n];
        let mut size = vec![0; n];
        for i in 0..n {
            id[i] = i;
            size[i] = 1;
        }
        Self { id, size, count: n }
    }

    /// Returns the parent of the element
    pub fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while x != self.id[x] {
            x = self.id[x];
            // self.id[x] = self.id[self.id[x]]; // path compression
        }
        x
    }

    /// Unions the sets containing x and y
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            self.id[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.id[y] = x;
            self.size[x] += self.size[y];
        }
        self.count -= 1;
        true
    }

    /// Checks if x and y are in the same set
    pub fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// Returns the number of disjoint sets
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(10);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(2), 2);
        assert_eq!(uf.find(3), 3);
        assert_eq!(uf.find(4), 4);
        assert_eq!(uf.find(5), 5);
        assert_eq!(uf.find(6), 6);
        assert_eq!(uf.find(7), 7);
        assert_eq!(uf.find(8), 8);
        assert_eq!(uf.find(9), 9);

        assert_eq!(uf.union(0, 1), true);
        assert_eq!(uf.union(1, 2), true);
        assert_eq!(uf.union(2, 3), true);
        assert_eq!(uf.union(3, 4), true);
        assert_eq!(uf.union(4, 5), true);
        assert_eq!(uf.union(5, 6), true);
        assert_eq!(uf.union(6, 7), true);
        assert_eq!(uf.union(7, 8), true);
        assert_eq!(uf.union(8, 9), true);
        assert_eq!(uf.union(9, 0), false);

        assert_eq!(1, uf.count());
    }
}
