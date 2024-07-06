// Author : cyrixninja
// Tree Sort Algorithm
// https://en.wikipedia.org/wiki/Tree_sort
// Wikipedia :A tree sort is a sort algorithm that builds a binary search tree from the elements to be sorted, and then traverses the tree (in-order) so that the elements come out in sorted order.
// Its typical use is sorting elements online: after each insertion, the set of elements seen so far is available in sorted order.

struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, value: T) {
        self.root = Some(Self::insert_recursive(self.root.take(), value));
    }

    fn insert_recursive(root: Option<Box<TreeNode<T>>>, value: T) -> Box<TreeNode<T>> {
        match root {
            None => Box::new(TreeNode::new(value)),
            Some(mut node) => {
                if value <= node.value {
                    node.left = Some(Self::insert_recursive(node.left.take(), value));
                } else {
                    node.right = Some(Self::insert_recursive(node.right.take(), value));
                }
                node
            }
        }
    }

    fn in_order_traversal(&self, result: &mut Vec<T>) {
        Self::in_order_recursive(&self.root, result);
    }

    fn in_order_recursive(root: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
        if let Some(node) = root {
            Self::in_order_recursive(&node.left, result);
            result.push(node.value.clone());
            Self::in_order_recursive(&node.right, result);
        }
    }
}

pub fn tree_sort<T: Ord + Clone>(arr: &mut Vec<T>) {
    let mut tree = BinarySearchTree::new();

    for elem in arr.iter().cloned() {
        tree.insert(elem);
    }

    let mut result = Vec::new();
    tree.in_order_traversal(&mut result);

    *arr = result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![8];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![8]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random() {
        let mut arr = vec![9, 6, 10, 11, 2, 19];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![2, 6, 9, 10, 11, 19]);
    }
}
