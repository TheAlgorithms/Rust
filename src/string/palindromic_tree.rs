use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct PalindromicTree {
    s: Vec<char>,
    tree: Tree,
    cur: Rc<RefCell<Node>>,
}

impl PalindromicTree {
    pub fn new() -> PalindromicTree {
        let tree = Tree::new();
        let cur = Rc::clone(&tree.odd_root);
        PalindromicTree {
            s: Vec::new(),
            tree,
            cur,
        }
    }

    pub fn input(&mut self, c: char) {
        while !self.accept_char(c) {
            let cur = self.cur.borrow().suffix();
            self.cur = cur;
        }
        self.s.push(c);
        if self.cur.borrow().children.contains_key(&c) {
            let cur = Rc::clone(&self.cur.borrow().children[&c]);
            self.cur = cur;
            return;
        }
        let parent = Rc::clone(&self.cur);
        let mut child = Node {
            len: parent.borrow().len + 2,
            suffix: Rc::downgrade(&self.tree.even_root),
            children: HashMap::new(),
        };
        let mut suffix_node = parent.borrow().suffix.clone();
        while let Some(suffix) = suffix_node.upgrade() {
            let suffix = suffix.borrow();
            if self.s[(self.s.len() as isize - suffix.len - 2) as usize] == c {
                child.suffix = Rc::downgrade(&suffix.children[&c]);
                break;
            }
            suffix_node = suffix.suffix.clone();
        }
        let child = Rc::new(RefCell::new(child));
        parent.borrow_mut().children.insert(c, Rc::clone(&child));
        self.tree.len += 1;
        self.cur = child;
    }

    pub fn longest_suffix_palindrome(&self) -> String {
        self.s[self.s.len() - self.longest_suffix_palindrome_length()..]
            .iter()
            .collect()
    }

    pub fn longest_suffix_palindrome_length(&self) -> usize {
        match self.cur.borrow().len {
            res if res < 0 => 0,
            res => res as usize,
        }
    }

    pub fn count_palindrome(&self) -> usize {
        self.tree.len()
    }

    fn accept_char(&self, c: char) -> bool {
        let idx = self.s.len() as isize - self.cur.borrow().len - 1;
        idx >= 0 && (idx as usize == self.s.len() || self.s[idx as usize] == c)
    }
}

impl Default for PalindromicTree {
    fn default() -> PalindromicTree {
        PalindromicTree::new()
    }
}

#[derive(Clone, Debug)]
struct Tree {
    odd_root: Rc<RefCell<Node>>,
    even_root: Rc<RefCell<Node>>,
    len: usize,
}

impl Tree {
    fn new() -> Tree {
        let odd_root = Node::odd_root();
        let even_root = Node::even_root(Rc::downgrade(&odd_root));
        Tree {
            odd_root,
            even_root,
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Clone, Debug)]
struct Node {
    len: isize,
    suffix: Weak<RefCell<Node>>,
    children: HashMap<char, Rc<RefCell<Node>>>,
}

impl Node {
    fn odd_root() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            len: -1,
            suffix: Weak::new(),
            children: HashMap::new(),
        }))
    }

    fn even_root(odd_root: Weak<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            len: 0,
            suffix: odd_root,
            children: HashMap::new(),
        }))
    }

    fn suffix(&self) -> Rc<RefCell<Node>> {
        self.suffix.upgrade().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_str(s: &str) -> (usize, String) {
        let mut longest_palindrome = String::new();
        let mut longest_palindrome_size = 0;
        let mut palindromic_tree = PalindromicTree::new();
        for c in s.chars() {
            palindromic_tree.input(c);
            let cur_len = palindromic_tree.longest_suffix_palindrome_length();
            if cur_len > longest_palindrome_size {
                longest_palindrome_size = cur_len;
                longest_palindrome = palindromic_tree.longest_suffix_palindrome();
            }
        }
        (palindromic_tree.count_palindrome(), longest_palindrome)
    }

    #[test]
    fn test_palindromic_tree() {
        assert_eq!(test_str(""), (0, String::from("")));
        assert_eq!(test_str("abba"), (4, String::from("abba")));
        assert_eq!(test_str("abbba"), (5, String::from("abbba")));
        assert_eq!(test_str("ababa"), (5, String::from("ababa")));
        assert_eq!(test_str("ccdaabba"), (8, String::from("abba")));
        assert_eq!(test_str("xxyyzz"), (6, String::from("xx")));
        assert_eq!(test_str("xxyyzzxxzzyy"), (10, String::from("yyzzxxzzyy")));
        assert_eq!(test_str("abcba"), (5, String::from("abcba")));
        assert_eq!(test_str("aabba"), (5, String::from("abba")));
    }
}
