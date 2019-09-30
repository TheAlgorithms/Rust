// Implementation of a binary tree
pub struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: i32,
}

impl Node {
    pub fn insert(&mut self, data: i32) {
        // If data to insert is already present in the tree, return
        if self.data == data {
            return;
        }
        // Left if smaller, right if bigger.
        let node = if data < self.data {
            &mut self.left
        } else {
            &mut self.right
        };
        match node {
            &mut Some(ref mut subnode) => subnode.insert(data), // recursively insert
            &mut None => {
                // Create new node if one doesn't exist yet
                let new_node = Node {
                    data,
                    left: None,
                    right: None,
                };
                *node = Some(Box::new(new_node))
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    #[test]
    pub fn test_insert() {
        let mut base = Node {
            left: None,
            right: None,
            data: 5,
        };
        base.insert(4);
        base.insert(6);
        assert_eq!(base.left.unwrap().data, 4);
        assert_eq!(base.right.unwrap().data, 6);
    }
}
