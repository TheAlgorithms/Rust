/// This struct implements as Binary Search Tree (BST), which is a
/// simple data structure for storing sorted data
pub struct BinarySearchTree<'a, T>
    where T: Ord
{
    value: Option<&'a T>,
    left: Option<Box<BinarySearchTree<'a, T>>>,
    right: Option<Box<BinarySearchTree<'a, T>>>,
}

impl<'a, T> BinarySearchTree<'a, T>
    where T: Ord
{
    /// Create a new, empty BST
    pub fn new<'b>() -> BinarySearchTree<'b, T> {
        BinarySearchTree { value: None, left: None, right: None }
    }

    /// Find a value in this tree. Returns True iff value is in this
    /// tree, and false otherwise
    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => {
                if *key == value {
                    true
                } else if *key > value {
                    match &self.left {
                        Some(node) => node.search(value),
                        None => false,
                    }
                } else {
                    match &self.right {
                        Some(node) => node.search(value),
                        None => false,
                    }
                }
            },
            None => false,
        }
    }

    /// Insert a value into the appropriate location in this tree.
    /// Returns true if the tree was updated (value was inserted) or
    /// false if the tree was not updated (because value was already
    /// present).
    pub fn insert(&mut self, value: &'a T) -> bool {
        if self.value.is_none() {
            self.value = Some(value);
            true
        } else {
            let key = self.value.unwrap();
            let target_node = if value < key {
                &mut self.left
            } else {
                &mut self.right
            };
            match target_node {
                &mut Some(ref mut node) => {
                    node.insert(value)
                },
                &mut None => {
                    let mut node = BinarySearchTree::new();
                    node.insert(value);
                    *target_node = Some(Box::new(node));
                    true
                },
            }
        }
    }

    /// Deletes a given value from this tree.
    /// Returns true iff the value to be removed was found in the tree.
    pub fn delete(&mut self, value: &T) -> bool {
        if self.value.is_none() {
            false
        } else {
            if self.value.unwrap() == value {
                match &mut self.left {
                    Some(node) => {
                        self.value = *node.maximum();
                        node.delete(value);
                    },
                    None => {
                        self.value = None;
                    },
                }
                true
            } else if self.value.unwrap() > value {
                match &mut self.left {
                    Some(node) => node.delete(value),
                    None => false,
                }
            } else {
                match &mut self.right {
                    Some(node) => node.delete(value),
                    None => true,
                }
            }
        }
    }

    /// Returns the smallest value in this tree
    pub fn minimum(&self) -> &Option<&'a T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => &self.value,
        }
    }

    /// Returns the largest value in this tree
    pub fn maximum(&self) -> &Option<&'a T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => &self.value,
        }
    }

    /// Returns the largest value in this tree smaller than value
    pub fn floor(&self, value: &T) -> &Option<&'a T> {
        match &self.value {
            Some(key) => {
                if *key > value {
                    match &self.left {
                        Some(node) => node.floor(value),
                        None => &None,
                    }
                } else if *key < value {
                    match &self.right {
                        Some(node) => {
                            let val = node.floor(value);
                            match &val {
                                Some(_) => &val,
                                None => &self.value,
                            }
                        },
                        None => &self.value,
                    }
                } else {
                    &self.value
                }
            },
            None => &self.value,
        }
    }

    /// Returns the smallest value in this tree larger than value
    pub fn ceil(&self, value: &T) -> &Option<&'a T> {
        match &self.value {
            Some(key) => {
                if *key < value {
                    match &self.right {
                        Some(node) => node.ceil(value),
                        None => &None,
                    }
                } else if *key > value {
                    match &self.left {
                        Some(node) => {
                            let val = node.ceil(value);
                            match &val {
                                Some(_) => &val,
                                None => &self.value,
                            }
                        },
                        None => &self.value,
                    }
                } else {
                    &self.value
                }
            },
            None => &self.value,
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<'static, &'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert(&"hello there");
        tree.insert(&"general kenobi");
        tree.insert(&"you are a bold one");
        tree.insert(&"kill him");
        tree.insert(&"back away...I will deal with this jedi slime myself");
        tree.insert(&"your move");
        tree.insert(&"you fool");
        tree
    }

    #[test]
    fn test_insert_and_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(!tree.search(&"but i was going to tosche station to pick up some power converters"));
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(*tree.minimum().unwrap(), "back away...I will deal with this jedi slime myself");
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(&0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(&-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(&5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_delete() {
        let mut tree = prequel_memes_tree();
        assert!(tree.search(&"you fool"));
        assert!(tree.delete(&"you fool"));
        assert!(!tree.search(&"you fool"));
        assert!(!tree.delete(&"you fool"));
        assert!(tree.search(&"hello there"));
        assert!(tree.delete(&"hello there"));
        assert!(!tree.search(&"hello there"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"your move"));
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(*tree.floor(&"these are not the droids you're looking for").unwrap(), "kill him");
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(*tree.floor(&"but i was going to tasche station").unwrap(), "back away...I will deal with this jedi slime myself");
        assert_eq!(*tree.floor(&"you underestimate my power").unwrap(), "you fool");
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(*tree.ceil(&"these are not the droids you're looking for").unwrap(), "you are a bold one");
        assert_eq!(*tree.ceil(&"another death star").unwrap(), "back away...I will deal with this jedi slime myself");
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(*tree.ceil(&"but i was going to tasche station").unwrap(), "general kenobi");
        assert_eq!(*tree.ceil(&"you underestimate my power").unwrap(), "your move");
        assert!(tree.ceil(&"your new empire").is_none());
    }
}

