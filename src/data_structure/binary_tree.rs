#[derive(PartialEq)]
pub struct Node<'a> {
    val: &'a str,
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn insert(&mut self, new_val: &'a str) {
        if self.val == new_val {
            return;
        }
        let target_node = match new_val < self.val {
            true => &mut self.left,
            false => &mut self.right,
        };
        match target_node {
            &mut Some(ref mut sub_node) => sub_node.insert(new_val),
            &mut None => {
                let new_node = Node {
                    val: new_val,
                    left: None,
                    right: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it_inserts_nodes() {
        let mut mode = Node {
            val: "q",
            left: None,
            right: None,
        };
        mode.insert("x");
        mode.insert("a");
        mode.insert("f");
        assert!(
            mode == Node {
                val: "q",
                left: Some(Box::new(Node {
                    val: "a",
                    left: None,
                    right: Some(Box::new(Node {
                        val: "f",
                        left: None,
                        right: None
                    })),
                })),
                right: Some(Box::new(Node {
                    val: "x",
                    left: None,
                    right: None
                })),
            }
        );
    }
}
