use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

// Why to need a different Struct for props...
// Check - http://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/#fnref:improvement
struct BTreeProps {
    degree: usize,
    max_keys: usize,
    min_keys: usize,
    mid_key_index: usize,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
        Node {
            keys: match _keys {
                Some(_keys) => _keys,
                None => Vec::with_capacity(degree - 1),
            },
            children: match _children {
                Some(_children) => _children,
                None => Vec::with_capacity(degree),
            },
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
}

impl BTreeProps {
    fn new(degree: usize) -> Self {
        BTreeProps {
            degree,
            max_keys: degree - 1,
            min_keys: degree / 2 - 1,
            mid_key_index: (degree - 1) / 2,
        }
    }

    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
        node.keys.len() == self.max_keys
    }

    // Split Child expects the Child Node to be full
    /// Move the middle_key to parent node and split the child_node's
    /// keys/chilren_nodes into half
    fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
        let child = &mut parent.children[child_index];
        let middle_key = child.keys[self.mid_key_index];
        let right_keys = match child.keys.split_off(self.mid_key_index).split_first() {
            Some((_first, _others)) => {
                // We don't need _first, as it will move to parent node.
                _others.to_vec()
            }
            None => Vec::with_capacity(self.max_keys),
        };
        let mut right_children = None;
        if !child.is_leaf() {
            right_children = Some(child.children.split_off(self.mid_key_index + 1));
        }
        let new_child_node: Node<T> = Node::new(self.degree, Some(right_keys), right_children);

        parent.keys.insert(child_index, middle_key);
        parent.children.insert(child_index + 1, new_child_node);
    }

    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
        let mut index: isize = isize::try_from(node.keys.len()).ok().unwrap() - 1;
        while index >= 0 && node.keys[index as usize] >= key {
            index -= 1;
        }

        let mut u_index: usize = usize::try_from(index + 1).ok().unwrap();
        if node.is_leaf() {
            // Just insert it, as we know this method will be called only when node is not full
            node.keys.insert(u_index, key);
        } else {
            if self.is_maxed_out(&node.children[u_index]) {
                self.split_child(node, u_index);
                if node.keys[u_index] < key {
                    u_index += 1;
                }
            }

            self.insert_non_full(&mut node.children[u_index], key);
        }
    }

    fn traverse_node<T: Ord + Debug>(&self, node: &Node<T>, depth: usize) {
        if node.is_leaf() {
            print!(" {0:{<1$}{2:?}{0:}<1$} ", "", depth, node.keys);
        } else {
            let _depth = depth + 1;
            for (index, key) in node.keys.iter().enumerate() {
                self.traverse_node(&node.children[index], _depth);
                // Check https://doc.rust-lang.org/std/fmt/index.html
                // And https://stackoverflow.com/a/35280799/2849127
                print!("{0:{<1$}{2:?}{0:}<1$}", "", depth, key);
            }
            self.traverse_node(&node.children.last().unwrap(), _depth);
        }
    }

    fn remove_from_node<T: Ord + Copy + Debug>(
        &self,
        node: &mut Node<T>,
        key: &T,
        is_root: bool,
    ) -> Option<T> {
        match node.keys.binary_search(&key) {
            Ok(index) => {
                if node.is_leaf() {
                    // For multiple instances of `key, `binary_search` method can return
                    // any possible index, thus removing in terms of index is more appropriate.
                    Some(node.keys.remove(index))
                } else {
                    self.remove_from_non_leaf(node, index, is_root)
                }
            }
            Err(_) => {
                if node.is_leaf() {
                    None
                } else {
                    let mut i: usize = 0; // Since here `i` can never be negetive,
                                          // and we are not subtracting it as well, without panic...
                    if i < node.keys.len() && *key > node.keys[i] {
                        i += 1;
                    }

                    let has_modified = self.repair_tree(node, i, is_root);
                    if has_modified {
                        self.remove_from_node(node, key, is_root)
                    } else {
                        self.remove_from_node(&mut node.children[i], key, false)
                    }
                }
            }
        }
    }

    /// Repairing a tree is needed, when the key is not found in the parent node,
    /// and may [not] exist in one of the child nodes. Even if the key exists or not in
    /// the child node, if the size of keys in any of the child nodes is less than (B - 1),
    /// we need to perform either Rotation or Merge operation on Parent and Child, to make the
    /// tree sane again, in terms of BTree rules.
    fn repair_tree<T: Ord + Copy>(
        &self,
        parent: &mut Node<T>,
        child_index: usize,
        is_root: bool,
    ) -> bool {
        let child = &parent.children[child_index];

        if child.keys.len() > self.min_keys && child.keys.len() <= self.max_keys {
            // Child has proper set of keys, no modifications needed, and we can
            // move further down the tree...
            return false;
        } else if child_index > 0 && parent.children[child_index - 1].keys.len() > self.min_keys {
            // We have a left sibling, which has enough keys to spare to child, thus we can rotate right
            self.rotate_right(parent, child_index);
            return true;
        } else if child_index < parent.children.len() - 1
            && parent.children[child_index + 1].keys.len() > self.min_keys
        {
            self.rotate_left(parent, child_index);
            return true;
        }

        if child_index > 0 {
            self.merge(parent, child_index - 1, child_index, is_root);
        } else {
            self.merge(parent, child_index, child_index + 1, is_root);
        }

        true
    }

    /// Move Last Key from left_child to Parent and Parent Key to Right Child
    /// Deletion will be taken care of in later cycles of recursion.
    fn rotate_right<T: Ord + Copy>(&self, parent: &mut Node<T>, child_index: usize) {
        let parent_key = parent.keys[child_index - 1];
        mem::replace(
            &mut parent.keys[child_index - 1],
            parent.children[child_index - 1].keys.pop().unwrap(),
        );
        parent.children[child_index].keys.insert(0, parent_key);

        // It can be possible that left_child key we removed is not a leaf, thus having right children, which
        // needs to be removed and put to right_child's new parent_key, as left_children.
        if !parent.children[child_index - 1].is_leaf() {
            let left_childs_rightmost_child =
                parent.children[child_index - 1].children.pop().unwrap();
            parent.children[child_index]
                .children
                .insert(0, left_childs_rightmost_child);
        }
    }

    fn rotate_left<T: Ord + Copy>(&self, parent: &mut Node<T>, child_index: usize) {
        let parent_key = parent.keys[child_index];
        let last_index = parent.children[child_index].keys.len() - 1;
        mem::replace(
            &mut parent.keys[child_index],
            parent.children[child_index + 1].keys.pop().unwrap(),
        );
        parent.children[child_index]
            .keys
            .insert(last_index, parent_key);

        if !parent.children[child_index + 1].is_leaf() {
            let right_childs_leftmost_child = parent.children[child_index + 1].children.remove(0);
            parent.children[child_index]
                .children
                .push(right_childs_leftmost_child);
        }
    }

    /// Merge Left Child, Parent and Right Child, maintaining the balance of BTree
    fn merge<T: Ord>(
        &self,
        parent: &mut Node<T>,
        left_child_index: usize,
        right_child_index: usize,
        is_root: bool,
    ) {
        let mut right_child = parent.children.remove(right_child_index);
        let parent_key = parent.keys.remove(left_child_index);
        let left_child: &mut Node<T>;

        // We need to take care of the root as well, is the parent node here is root
        if is_root && parent.keys.len() == 0 {
            let temp_left_child = parent.children.remove(left_child_index);
            mem::replace(parent, temp_left_child);
            left_child = parent;
        } else {
            left_child = &mut parent.children[left_child_index];
        }

        left_child.keys.push(parent_key);
        left_child.keys.append(&mut right_child.keys);

        if !left_child.is_leaf() && !right_child.is_leaf() {
            left_child.children.append(&mut right_child.children);
        }
    }

    // Key is found in the node, but it's not a leaf, we need to fix it as per BTree rules
    // and remove the key from the node
    fn remove_from_non_leaf<T: Ord + Copy + Debug>(
        &self,
        node: &mut Node<T>,
        key_index: usize,
        is_root: bool,
    ) -> Option<T> {
        let key = node.keys[key_index]; // This creates a Copy, and using an immutable ref here is impossible.
        let replacement_key: Option<T>;

        println!(
            "Inside non leaf: left size: {}, right size: {}, min size: {}",
            node.children[key_index].keys.len(),
            node.children[key_index + 1].keys.len(),
            self.min_keys
        );
        if node.children[key_index].keys.len() > self.min_keys {
            // Left Child has enough keys for replacement...
            replacement_key = self.find_largest_predecessor(&mut node.children[key_index]);
        } else if node.children[key_index + 1].keys.len() > self.min_keys {
            replacement_key = self.find_minimum_successor(&mut node.children[key_index + 1]);
        } else {
            self.merge(node, key_index, key_index + 1, is_root);
            return self.remove_from_node(node, &key, is_root);
        }

        println!("Inside non leaf: {:?}", replacement_key);
        if let Some(replacement) = replacement_key {
            mem::replace(&mut node.keys[key_index], replacement);
        };
        Some(key)
    }

    fn find_largest_predecessor<T: Ord + Copy>(&self, left_node: &mut Node<T>) -> Option<T> {
        if left_node.is_leaf() {
            left_node.keys.pop()
        } else {
            let last_index = left_node.children.len() - 1;
            // Left node is always a child, so is_root -> false always
            self.repair_tree(left_node, last_index, false);
            self.find_largest_predecessor(&mut left_node.children[last_index])
        }
    }

    fn find_minimum_successor<T: Ord + Copy>(&self, right_node: &mut Node<T>) -> Option<T> {
        println!("Inside min successor");
        if right_node.is_leaf() {
            Some(right_node.keys.remove(0))
        } else {
            self.repair_tree(right_node, 0, false);
            self.find_minimum_successor(&mut right_node.children[0])
        }
    }
}

impl<T> BTree<T>
where
    T: Ord + Copy + Debug + Default,
{
    pub fn new(branch_factor: usize) -> Self {
        let degree = 2 * branch_factor;
        BTree {
            root: Node::new(degree, None, None),
            props: BTreeProps::new(degree),
        }
    }

    pub fn insert(&mut self, key: T) {
        if self.props.is_maxed_out(&self.root) {
            // Create an empty root and split the old root...
            let mut new_root = Node::new(self.props.degree, None, None);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            self.props.split_child(&mut self.root, 0);
        }
        self.props.insert_non_full(&mut self.root, key);
    }

    pub fn traverse(&self) {
        self.props.traverse_node(&self.root, 0);
        println!("");
    }

    pub fn search(&self, key: T) -> bool {
        let mut current_node = &self.root;
        let mut index: isize;
        loop {
            index = isize::try_from(current_node.keys.len()).ok().unwrap() - 1;
            while index >= 0 && current_node.keys[index as usize] > key {
                index -= 1;
            }

            let u_index: usize = usize::try_from(index + 1).ok().unwrap();
            if index >= 0 && current_node.keys[u_index - 1] == key {
                break true;
            } else if current_node.is_leaf() {
                break false;
            } else {
                current_node = &current_node.children[u_index];
            }
        }
    }

    // Return the removed key
    pub fn remove(&mut self, key: &T) -> Option<T> {
        self.props.remove_from_node(&mut self.root, key, true)
    }
}

#[cfg(test)]
mod test {
    use super::BTree;

    #[test]
    fn test_search() {
        let mut tree = BTree::new(2);
        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(5);
        tree.insert(6);
        tree.insert(7);
        tree.insert(11);
        tree.insert(12);
        tree.insert(15);
        assert!(tree.search(15));
        assert_eq!(tree.search(16), false);
    }

    #[test]
    fn test_delete() {
        let mut tree = BTree::new(2);
        tree.insert(10);
        tree.insert(20);
        tree.insert(30);
        tree.insert(5);
        tree.insert(6);
        tree.insert(7);
        tree.insert(11);
        tree.insert(12);
        tree.insert(15);
        assert_eq!(tree.remove(&5), Some(5));
        assert_eq!(tree.search(5), false);
        assert_eq!(tree.remove(&16), None);
    }
}
