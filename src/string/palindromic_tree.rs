use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct PalindromicTree {
    s: Vec<char>,
    tree: Tree,
    cur: Weak<RefCell<Node>>,
}

impl PalindromicTree {
    pub fn new() -> PalindromicTree {
        let mut res = PalindromicTree {
            s: Vec::new(),
            tree: Tree::new(),
            cur: Weak::new(),
        };
        res.cur = Rc::downgrade(&res.tree.odd_root);
        res
    }

    pub fn input(&mut self, c: char) {
        let len = self.s.len();
        self.s.push(c);
        let mut idx = len as isize - self.cur().borrow().len - 1;
        while idx < 0 || self.s[idx as usize] != c {
            self.cur = self.cur().borrow().suffix.clone().unwrap();
            idx = len as isize - self.cur().borrow().len - 1;
        }
        if self.cur().borrow().children.contains_key(&c) {
            self.cur = Rc::downgrade(&self.cur().borrow().children[&c]);
            return;
        }
        let mut suffix_node = self.cur().borrow().suffix.clone();
        let suffix = loop {
            match suffix_node.map(|suffix| suffix.upgrade().unwrap()) {
                Some(suf) if !suf.borrow().children.contains_key(&c) => {
                    suffix_node = suf.borrow().suffix.clone();
                }
                Some(suf) => break suf.borrow().children[&c].clone(),
                None => break self.tree.even_root.clone(),
            }
        };
        let leaf = Rc::new(RefCell::new(Node {
            len: self.cur().borrow().len + 2,
            suffix: Some(Rc::downgrade(&suffix)),
            children: HashMap::new(),
        }));
        self.cur().borrow_mut().children.insert(c, Rc::clone(&leaf));
        self.cur = Rc::downgrade(&leaf);
        self.tree.len += 1;
    }

    pub fn longest_suffix_palindrome(&self) -> String {
        self.s[self.s.len() - self.longest_suffix_palindrome_length()..]
            .iter()
            .collect()
    }

    pub fn longest_suffix_palindrome_length(&self) -> usize {
        match self.cur().borrow().len {
            res if res < 0 => 0,
            res => res as usize,
        }
    }

    pub fn count_palindrome(&self) -> usize {
        self.tree.len()
    }

    fn cur(&self) -> Rc<RefCell<Node>> {
        self.cur.upgrade().unwrap()
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
    suffix: Option<Weak<RefCell<Node>>>,
    children: HashMap<char, Rc<RefCell<Node>>>,
}

impl Node {
    fn odd_root() -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            len: -1,
            suffix: None,
            children: HashMap::new(),
        }))
    }

    fn even_root(odd_root: Weak<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            len: 0,
            suffix: Some(odd_root),
            children: HashMap::new(),
        }))
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
        assert_eq!(test_str("ccdaabba"), (8, String::from("abba")));
        assert_eq!(test_str("xxyyzz"), (6, String::from("xx")));
        assert_eq!(test_str("xxyyzzxxzzyy"), (10, String::from("yyzzxxzzyy")));
        assert_eq!(test_str("abcba"), (5, String::from("abcba")));
        assert_eq!(test_str("aabba"), (5, String::from("abba")));
    }
}
