// This struct implements Van Emde Boas tree (VEB tree). It stores integers in range [0, U), where
// O is any integer that is a power of 2. It supports operations such as insert, search,
// predecessor, and successor in O(log(log(U))) time. The structure takes O(U) space.
pub struct VebTree {
    size: u32,
    child_size: u32, // Set to square root of size. Cache here to avoid recomputation.
    min: u32,
    max: u32,
    summary: Option<Box<VebTree>>,
    cluster: Vec<VebTree>,
}

impl VebTree {
    /// Create a new, empty VEB tree. The tree will contain number of elements equal to size
    /// rounded up to the nearest power of two.
    pub fn new(size: u32) -> VebTree {
        let rounded_size = size.next_power_of_two();
        let child_size = (size as f64).sqrt().ceil() as u32;

        let mut cluster = Vec::new();
        if rounded_size > 2 {
            for _ in 0..rounded_size {
                cluster.push(VebTree::new(child_size));
            }
        }

        VebTree {
            size: rounded_size,
            child_size,
            min: u32::MAX,
            max: u32::MIN,
            cluster,
            summary: if rounded_size <= 2 {
                None
            } else {
                Some(Box::new(VebTree::new(child_size)))
            },
        }
    }

    fn high(&self, value: u32) -> u32 {
        value / self.child_size
    }

    fn low(&self, value: u32) -> u32 {
        value % self.child_size
    }

    fn index(&self, cluster: u32, offset: u32) -> u32 {
        cluster * self.child_size + offset
    }

    pub fn min(&self) -> u32 {
        self.min
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn iter(&self) -> VebTreeIter {
        VebTreeIter::new(self)
    }

    // A VEB tree is empty if the min is greater than the max.
    pub fn empty(&self) -> bool {
        self.min > self.max
    }

    // Returns true if value is in the tree, false otherwise.
    pub fn search(&self, value: u32) -> bool {
        if self.empty() {
            return false;
        } else if value == self.min || value == self.max {
            return true;
        } else if value < self.min || value > self.max {
            return false;
        }
        self.cluster[self.high(value) as usize].search(self.low(value))
    }

    fn insert_empty(&mut self, value: u32) {
        assert!(self.empty(), "tree should be empty");
        self.min = value;
        self.max = value;
    }

    // Inserts value into the tree.
    pub fn insert(&mut self, mut value: u32) {
        assert!(value < self.size);

        if self.empty() {
            self.insert_empty(value);
            return;
        }

        if value < self.min {
            // If the new value is less than the current tree's min, set the min to the new value
            // and insert the old min.
            (value, self.min) = (self.min, value);
        }

        if self.size > 2 {
            // Non base case. The checks for min/max will handle trees of size 2.
            let high = self.high(value);
            let low = self.low(value);
            if self.cluster[high as usize].empty() {
                // If the cluster tree for the value is empty, we set the min/max of the tree to
                // value and record that the cluster tree has an elements in the summary.
                self.cluster[high as usize].insert_empty(low);
                if let Some(summary) = self.summary.as_mut() {
                    summary.insert(high);
                }
            } else {
                // If the cluster tree already has a value, the summary does not need to be
                // updated. Recursively insert the value into the cluster tree.
                self.cluster[high as usize].insert(low);
            }
        }

        if value > self.max {
            self.max = value;
        }
    }

    // Returns the next greatest value(successor) in the tree after pred. Returns
    // `None` if there is no successor.
    pub fn succ(&self, pred: u32) -> Option<u32> {
        if self.empty() {
            return None;
        }

        if self.size == 2 {
            // Base case. If pred is 0, and 1 exists in the tree (max is set to 1), the successor
            // is 1.
            return if pred == 0 && self.max == 1 {
                Some(1)
            } else {
                None
            };
        }

        if pred < self.min {
            // If the predecessor is less than the minimum of this tree, the successor is the min.
            return Some(self.min);
        }

        let low = self.low(pred);
        let high = self.high(pred);

        if !self.cluster[high as usize].empty() && low < self.cluster[high as usize].max {
            // The successor is within the same cluster as the predecessor
            return Some(self.index(high, self.cluster[high as usize].succ(low).unwrap()));
        };

        // If we reach this point, the successor exists in a different cluster. We use the summary
        // to efficiently query which cluster the successor lives in. If there is no successor
        // cluster, return None.
        let succ_cluster = self.summary.as_ref().unwrap().succ(high);
        succ_cluster
            .map(|succ_cluster| self.index(succ_cluster, self.cluster[succ_cluster as usize].min))
    }

    // Returns the next smallest value(predecessor) in the tree after succ. Returns
    // `None` if there is no predecessor. pred() is almost a mirror of succ().
    // Differences are noted in comments.
    pub fn pred(&self, succ: u32) -> Option<u32> {
        if self.empty() {
            return None;
        }

        // base case.
        if self.size == 2 {
            return if succ == 1 && self.min == 0 {
                Some(0)
            } else {
                None
            };
        }

        if succ > self.max {
            return Some(self.max);
        }

        let low = self.low(succ);
        let high = self.high(succ);

        if !self.cluster[high as usize].empty() && low > self.cluster[high as usize].min {
            return Some(self.index(high, self.cluster[high as usize].pred(low).unwrap()));
        };

        // Find the cluster that has the predecessor. The successor will be that cluster's max.
        let succ_cluster = self.summary.as_ref().unwrap().pred(high);
        match succ_cluster {
            Some(succ_cluster) => {
                Some(self.index(succ_cluster, self.cluster[succ_cluster as usize].max))
            }
            // Special case for pred() that does not exist in succ(). The current tree's min
            // does not exist in a cluster. So if we cannot find a cluster that could have the
            // predecessor, the predecessor could be the min of the current tree.
            None => {
                if succ > self.min {
                    Some(self.min)
                } else {
                    None
                }
            }
        }
    }
}

pub struct VebTreeIter<'a> {
    tree: &'a VebTree,
    curr: Option<u32>,
}

impl<'a> VebTreeIter<'a> {
    pub fn new(tree: &'a VebTree) -> VebTreeIter {
        let curr = if tree.empty() { None } else { Some(tree.min) };
        VebTreeIter { tree, curr }
    }
}

impl<'a> Iterator for VebTreeIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let curr = self.curr;
        curr?;
        self.curr = self.tree.succ(curr.unwrap());
        curr
    }
}

#[cfg(test)]
mod test {
    use super::VebTree;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn test_veb_tree(size: u32, mut elements: Vec<u32>, exclude: Vec<u32>) {
        // Insert elements
        let mut tree = VebTree::new(size);
        for element in elements.iter() {
            tree.insert(*element);
        }

        // Test search
        for element in elements.iter() {
            assert!(tree.search(*element));
        }
        for element in exclude {
            assert!(!tree.search(element));
        }

        // Test iterator and successor, and predecessor
        elements.sort();
        elements.dedup();
        for (i, element) in tree.iter().enumerate() {
            assert!(elements[i] == element);
        }
        for i in 1..elements.len() {
            assert!(tree.succ(elements[i - 1]) == Some(elements[i]));
            assert!(tree.pred(elements[i]) == Some(elements[i - 1]));
        }
    }

    #[test]
    fn test_empty() {
        test_veb_tree(16, Vec::new(), (0..16).collect());
    }

    #[test]
    fn test_single() {
        test_veb_tree(16, Vec::from([5]), (0..16).filter(|x| *x != 5).collect());
    }

    #[test]
    fn test_two() {
        test_veb_tree(
            16,
            Vec::from([4, 9]),
            (0..16).filter(|x| *x != 4 && *x != 9).collect(),
        );
    }

    #[test]
    fn test_repeat_insert() {
        let mut tree = VebTree::new(16);
        for _ in 0..5 {
            tree.insert(10);
        }
        assert!(tree.search(10));
        let elements: Vec<u32> = (0..16).filter(|x| *x != 10).collect();
        for element in elements {
            assert!(!tree.search(element));
        }
    }

    #[test]
    fn test_linear() {
        test_veb_tree(16, (0..10).collect(), (10..16).collect());
    }

    fn test_full(size: u32) {
        test_veb_tree(size, (0..size).collect(), Vec::new());
    }

    #[test]
    fn test_full_small() {
        test_full(8);
        test_full(10);
        test_full(16);
        test_full(20);
        test_full(32);
    }

    #[test]
    fn test_full_256() {
        test_full(256);
    }

    #[test]
    fn test_10_256() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..10).map(|_| rng.gen_range(0..255)).collect();
        test_veb_tree(256, elements, Vec::new());
    }

    #[test]
    fn test_100_256() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..100).map(|_| rng.gen_range(0..255)).collect();
        test_veb_tree(256, elements, Vec::new());
    }

    #[test]
    fn test_100_300() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..100).map(|_| rng.gen_range(0..255)).collect();
        test_veb_tree(300, elements, Vec::new());
    }
}
