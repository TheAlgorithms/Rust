use std::cmp::Ordering;

/// A self-adjusting binary search tree.
///
/// Splay trees move recently accessed values to the root. They do not keep a
/// strict height-balance invariant, but provide amortized logarithmic access
/// time over a sequence of operations.
pub struct SplayTree<T: Ord> {
    root: Link<T>,
    len: usize,
}

struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

type BoxedNode<T> = Box<Node<T>>;
type Link<T> = Option<BoxedNode<T>>;

enum SplayCase<T> {
    Found {
        root: BoxedNode<T>,
    },

    StopLeft {
        root: BoxedNode<T>,
    },

    StopRight {
        root: BoxedNode<T>,
    },

    Left {
        root: BoxedNode<T>,
        left: BoxedNode<T>,
    },

    LeftLeft {
        root: BoxedNode<T>,
        left: BoxedNode<T>,
    },

    LeftRight {
        root: BoxedNode<T>,
        left: BoxedNode<T>,
    },

    Right {
        root: BoxedNode<T>,
        right: BoxedNode<T>,
    },

    RightLeft {
        root: BoxedNode<T>,
        right: BoxedNode<T>,
    },

    RightRight {
        root: BoxedNode<T>,
        right: BoxedNode<T>,
    },
}

impl<T: Ord> SplayTree<T> {
    /// Creates an empty `SplayTree`.
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns the number of values in the tree.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the tree contains no values.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if the tree contains `value`.
    ///
    /// This operation splays the matching value to the root. If `value` is not
    /// present, the last visited node is splayed to the root instead.
    pub fn contains(&mut self, value: &T) -> bool {
        let old_root = self.root.take();
        self.root = Self::splay(old_root, value);

        match &self.root {
            Some(node) => node.value == *value,
            _ => false,
        }
    }

    /// Inserts `value` into the tree.
    ///
    /// Returns `true` if the value was not already present. After the operation,
    /// the inserted value, or the existing matching value, is at the root.
    pub fn insert(&mut self, value: T) -> bool {
        let is_contains = self.contains(&value);

        if is_contains {
            return false;
        }

        self.len += 1;
        let mut new_root = Box::new(Node {
            value,
            left: None,
            right: None,
        });
        if let Some(mut root_node) = self.root.take() {
            match new_root.value.cmp(&root_node.value) {
                Ordering::Less => {
                    new_root.left = root_node.left.take();
                    new_root.right = Some(root_node);
                }
                Ordering::Greater => {
                    new_root.right = root_node.right.take();
                    new_root.left = Some(root_node);
                }
                Ordering::Equal => {
                    // unreachable
                }
            }
        }
        self.root = Some(new_root);
        true
    }

    /// Merges `other` into this tree.
    ///
    /// This method assumes that every value in this tree is less than every
    /// value in `other`. If this precondition is not met, the binary search tree
    /// invariant may be broken.
    pub fn merge(&mut self, other: SplayTree<T>) {
        self.len += other.len();
        self.merge_with_node_root(other.root);
    }

    /// Removes `value` from the tree and returns it if it was present.
    ///
    /// If the value is found, it is first splayed to the root, then the
    /// remaining left and right subtrees are merged.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let is_contains = self.contains(value);
        if is_contains {
            self.len -= 1;
            let root = self.root.take()?;
            let answer = root.value;
            let left = root.left;
            let right = root.right;
            self.root = left;
            self.merge_with_node_root(right);
            Some(answer)
        } else {
            None
        }
    }

    fn merge_with_node_root(&mut self, other_root: Link<T>) {
        self.splay_max();

        match &mut self.root {
            Some(root) => root.right = other_root,
            None => {
                self.root = other_root;
            }
        }
    }

    fn splay_max(&mut self) {
        if let Some(root) = self.root.take() {
            self.root = Some(Self::splay_max_unwrapped(root));
        }
    }

    fn splay_max_unwrapped(mut root: BoxedNode<T>) -> BoxedNode<T> {
        let Some(mut right) = root.right.take() else {
            return root;
        };

        if let Some(right_right) = right.right.take() {
            right.right = Some(Self::splay_max_unwrapped(right_right));

            let new_root = Self::rotate_left_with_right(root, right);
            Self::rotate_left_if_possible(new_root)
        } else {
            Self::rotate_left_with_right(root, right)
        }
    }

    fn splay(root: Link<T>, key: &T) -> Link<T> {
        let root = root?;
        Some(Self::splay_unwrapped(root, key))
    }

    fn splay_unwrapped(root: BoxedNode<T>, key: &T) -> BoxedNode<T> {
        let class = Self::classify(root, key);

        match class {
            SplayCase::Found { root }
            | SplayCase::StopLeft { root }
            | SplayCase::StopRight { root } => root,
            SplayCase::Left { root, left } => Self::rotate_right_with_left(root, left),
            SplayCase::LeftLeft { root, mut left } => {
                if let Some(left_left) = left.left.take() {
                    left.left = Some(Self::splay_unwrapped(left_left, key));
                }
                let parent_root = Self::rotate_right_with_left(root, left);
                Self::rotate_right_if_possible(parent_root)
            }
            SplayCase::LeftRight { root, mut left } => {
                if let Some(left_right) = left.right.take() {
                    left.right = Some(Self::splay_unwrapped(left_right, key));
                }
                let new_left = Self::rotate_left_if_possible(left);
                Self::rotate_right_with_left(root, new_left)
            }
            SplayCase::Right { root, right } => Self::rotate_left_with_right(root, right),
            SplayCase::RightLeft { root, mut right } => {
                if let Some(right_left) = right.left.take() {
                    right.left = Some(Self::splay_unwrapped(right_left, key));
                }
                let new_right = Self::rotate_right_if_possible(right);
                Self::rotate_left_with_right(root, new_right)
            }
            SplayCase::RightRight { root, mut right } => {
                if let Some(right_right) = right.right.take() {
                    right.right = Some(Self::splay_unwrapped(right_right, key));
                }
                let parent_root = Self::rotate_left_with_right(root, right);
                Self::rotate_left_if_possible(parent_root)
            }
        }
    }

    fn classify(mut root: BoxedNode<T>, key: &T) -> SplayCase<T> {
        match key.cmp(&root.value) {
            Ordering::Equal => SplayCase::Found { root },
            Ordering::Less => {
                let Some(left) = root.left.take() else {
                    return SplayCase::StopLeft { root };
                };

                match key.cmp(&left.value) {
                    Ordering::Equal => SplayCase::Left { root, left },
                    Ordering::Less => SplayCase::LeftLeft { root, left },
                    Ordering::Greater => SplayCase::LeftRight { root, left },
                }
            }
            Ordering::Greater => {
                let Some(right) = root.right.take() else {
                    return SplayCase::StopRight { root };
                };

                match key.cmp(&right.value) {
                    Ordering::Equal => SplayCase::Right { root, right },
                    Ordering::Less => SplayCase::RightLeft { root, right },
                    Ordering::Greater => SplayCase::RightRight { root, right },
                }
            }
        }
    }

    fn rotate_right_with_left(mut root: BoxedNode<T>, mut left: BoxedNode<T>) -> BoxedNode<T> {
        root.left = left.right.take();
        left.right = Some(root);
        left
    }

    fn rotate_left_with_right(mut root: BoxedNode<T>, mut right: BoxedNode<T>) -> BoxedNode<T> {
        root.right = right.left.take();
        right.left = Some(root);
        right
    }

    fn rotate_right_if_possible(mut root: BoxedNode<T>) -> BoxedNode<T> {
        let Some(left) = root.left.take() else {
            return root;
        };
        Self::rotate_right_with_left(root, left)
    }

    fn rotate_left_if_possible(mut root: BoxedNode<T>) -> BoxedNode<T> {
        let Some(right) = root.right.take() else {
            return root;
        };
        Self::rotate_left_with_right(root, right)
    }
}

impl<T: Ord> Default for SplayTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for SplayTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = SplayTree::new();
        for value in iter {
            tree.insert(value);
        }
        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_in_order<'a>(node: &'a Link<i32>, values: &mut Vec<&'a i32>) {
        if let Some(node) = node {
            collect_in_order(&node.left, values);
            values.push(&node.value);
            collect_in_order(&node.right, values);
        }
    }

    fn assert_bst_invariant(tree: &SplayTree<i32>) {
        let mut values = Vec::new();
        collect_in_order(&tree.root, &mut values);

        assert_eq!(values.len(), tree.len());

        for pair in values.windows(2) {
            assert!(
                pair[0] < pair[1],
                "in-order traversal must be strictly sorted"
            );
        }
    }

    #[test]
    fn new_tree_is_empty() {
        let tree = SplayTree::<i32>::new();

        assert_eq!(tree.len(), 0);
        assert!(tree.is_empty());
    }

    #[test]
    fn simple_insert_contains() {
        let mut tree = SplayTree::<i32>::new();

        tree.insert(1);

        assert!(tree.contains(&1));
    }

    #[test]
    fn insert_contains() {
        let mut tree: SplayTree<i32> = (0..10).collect();

        for i in 0..10 {
            assert!(!tree.insert(i));
            assert!(tree.contains(&i));
        }

        for i in -10..0 {
            assert!(!tree.contains(&i));
        }

        assert_eq!(tree.len(), 10);
    }

    #[test]
    fn mixed_accesses_keep_recent_items_near_root() {
        let mut tree = (0..5000).collect::<SplayTree<i32>>();

        for _ in 0..100 {
            for x in [4000, 2000, 1000, 2000, 3000, 4000] {
                assert!(tree.contains(&x));
            }
        }
    }

    #[test]
    fn merge_splay_trees() {
        let mut tree1 = (0..100).collect::<SplayTree<i32>>();
        let tree2 = (100..200).collect::<SplayTree<i32>>();

        tree1.merge(tree2);

        for i in 0..200 {
            assert!(tree1.contains(&i));
        }

        assert_eq!(tree1.len(), 200);

        assert_eq!(tree1.remove(&15), Some(15));
        assert_eq!(tree1.remove(&15), None);
        assert_eq!(tree1.len(), 199);
    }

    #[test]
    fn remove() {
        let mut tree = (0..100).collect::<SplayTree<i32>>();

        tree.remove(&32);

        assert!(!tree.contains(&32));

        for i in 0..32 {
            assert!(tree.contains(&i));
        }

        for i in 33..100 {
            assert!(tree.contains(&i));
        }

        assert_eq!(tree.len(), 99);
    }

    #[test]
    fn operations_keep_bst_invariant() {
        let mut tree = SplayTree::<i32>::new();

        for value in [10, 4, 20, 2, 8, 15, 30, 6, 9] {
            assert!(tree.insert(value));
            assert_bst_invariant(&tree);
        }

        for value in [6, 30, 10, 3, 21] {
            tree.contains(&value);
            assert_bst_invariant(&tree);
        }

        assert_eq!(tree.remove(&8), Some(8));
        assert_bst_invariant(&tree);

        assert_eq!(tree.remove(&100), None);
        assert_bst_invariant(&tree);

        let right_tree = [35, 40, 45, 50].into_iter().collect::<SplayTree<i32>>();
        tree.merge(right_tree);
        assert_bst_invariant(&tree);
    }
}
