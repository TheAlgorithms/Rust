use std::cell::RefCell;
use std::rc::Rc;

/**
 * Linked List Cycle Detection using Floyd's Tortoise and Hare Algorithm (https://en.wikipedia.org/wiki/Cycle_detection)
 *
 * This program demonstrates the use of Floyd's Tortoise and Hare Algorithm to detect a cycle
 * in a linked list. It defines a LinkedList struct and a Node struct to represent the linked list
 * and its nodes.
 *
 * The LinkedList struct provides methods for iterating through the list and finding a cycle,
 * while the Node struct represents individual nodes in the list.
 *
 * Author: [Gyandeep] (https://github.com/Gyan172004)
 */

// Define a linked list struct.
#[derive(Debug)]
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    curr: Option<Rc<RefCell<Node>>>,
}

// Implement an iterator for the linked list.
impl Iterator for LinkedList {
    type Item = Rc<RefCell<Node>>;
    fn next(&mut self) -> Option<Rc<RefCell<Node>>> {
        match &self.curr.clone() {
            Some(n) => match &RefCell::borrow(n).next {
                Some(n_next) => {
                    self.curr = Some(n_next.clone());
                    Some(n_next.clone())
                }
                None => None,
            },
            None => None,
        }
    }
}

// Implement methods for the linked list.
impl LinkedList {
    // Find a loop in the linked list using Floyd's Tortoise and Hare Algorithm.
    fn find_loop(&mut self) -> Option<usize> {
        let one_iter = self.head.clone();
        let mut next = one_iter.as_ref().unwrap().clone();
        for next_two in self.step_by(2) {
            let next_two_id = RefCell::borrow(&next_two).id;
            if next_two_id == RefCell::borrow(&next).id {
                return Some(next_two_id);
            }
            next = RefCell::borrow(&next.clone())
                .next
                .as_ref()
                .unwrap()
                .clone();
        }
        None
    }
}

// Define a node struct for the linked list.
#[derive(Debug)]
struct Node {
    id: usize,
    next: Option<Rc<RefCell<Node>>>,
}

// Implement methods for the node struct.
impl Node {
    fn new(id: usize) -> Self {
        Node { id, next: None }
    }
}

// Generate a linked list with a loop for testing.
fn gen_linked_list_with_loop() -> LinkedList {
    let mut head = Node::new(0);
    let mut node1 = Node::new(1);
    let node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let mut node4 = Node::new(4);
    let mut node5 = Node::new(5);
    let node2_link = Rc::new(RefCell::new(node2));
    node5.next = Some(node2_link.clone());
    node4.next = Some(Rc::new(RefCell::new(node5)));
    node3.next = Some(Rc::new(RefCell::new(node4)));
    node2_link.borrow_mut().next = Some(Rc::new(RefCell::new(node3)));
    node1.next = Some(node2_link.clone());
    head.next = Some(Rc::new(RefCell::new(node1)));

    let head_link = Rc::new(RefCell::new(head));
    LinkedList {
        head: Some(head_link.clone()),
        curr: Some(head_link.clone()),
    }
}

// Generate a linked list without a loop for testing.
fn gen_linked_list() -> LinkedList {
    let mut head = Node::new(0);
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let mut node4 = Node::new(4);
    let mut node5 = Node::new(5);
    node5.next = None;
    node4.next = Some(Rc::new(RefCell::new(node5)));
    node3.next = Some(Rc::new(RefCell::new(node4)));
    node2.next = Some(Rc::new(RefCell::new(node3)));
    node1.next = Some(Rc::new(RefCell::new(node2)));
    head.next = Some(Rc::new(RefCell::new(node1)));

    let head_link = Rc::new(RefCell::new(head));
    LinkedList {
        head: Some(head_link.clone()),
        curr: Some(head_link.clone()),
    }
}

// Check and print the result of loop detection.
fn check_result(result: Option<usize>) {
    match result {
        Some(p) => {
            println!("Iterator meet at:{}, loop found.", p)
        }
        None => {
            println!("No Loop found")
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop() {
        let mut linked_list_with_loop = gen_linked_list_with_loop();
        let mut linked_list = gen_linked_list();
        assert_eq!(linked_list.find_loop(), None);
        assert_eq!(linked_list_with_loop.find_loop(), Some(3));
    }
}
