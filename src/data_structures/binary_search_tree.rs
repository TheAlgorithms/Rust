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
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    #[test]
    fn test_insert_and_search() {
        let s1 = String::from("hello there");
        let s2 = String::from("general kenobi");
        let s3 = String::from("you are a bold one");
        let s4 = String::from("kill him");
        let s5 = String::from("back away...I will deal with this jedi slime myself");
        let s6 = String::from("your move");
        let s7 = String::from("you fool");
        let mut tree: BinarySearchTree<String> = BinarySearchTree::new();
        tree.insert(&s1);
        tree.insert(&s2);
        tree.insert(&s3);
        tree.insert(&s4);
        tree.insert(&s5);
        tree.insert(&s6);
        tree.insert(&s7);
        assert!(tree.search(&String::from("hello there")));
        assert!(tree.search(&String::from("you are a bold one")));
        assert!(tree.search(&String::from("general kenobi")));
        assert!(tree.search(&String::from("you fool")));
        assert!(tree.search(&String::from("kill him")));
        assert!(!tree.search(&String::from("but i was going to tosche station to pick up some power converters")));
        assert!(!tree.search(&String::from("only a sith deals in absolutes")));
        assert!(!tree.search(&String::from("you underestimate my power")));
    }
}

