use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::rc::{Rc, Weak};

pub struct AhoCorasick {
    root: Rc<RefCell<Node>>,
    word_count: usize,
}

impl Default for AhoCorasick {
    fn default() -> AhoCorasick {
        AhoCorasick::new()
    }
}

impl AhoCorasick {
    pub fn new() -> AhoCorasick {
        AhoCorasick {
            root: Rc::new(RefCell::new(Node::new())),
            word_count: 0,
        }
    }

    pub fn add_word(&mut self, word: &str) -> bool {
        let mut cur = Rc::clone(&self.root);
        for c in word.chars() {
            let parent_node = Rc::clone(&cur);
            if let Some(child) = parent_node.borrow().children.get(&c) {
                cur = Rc::clone(child);
                continue;
            }

            let mut child_node = Node::new();
            child_node.suffix = Rc::downgrade(&self.root);
            child_node.height = parent_node.borrow().height + c.len_utf8();
            let mut suffix = parent_node.borrow().suffix.upgrade();
            while let Some(ref suf) = suffix.clone() {
                match suf.borrow().children.get(&c) {
                    Some(node) => {
                        child_node.suffix = Rc::downgrade(&node);
                        break;
                    }
                    None => suffix = suf.borrow().suffix.upgrade(),
                }
            }
            cur = Rc::new(RefCell::new(child_node));
            parent_node.borrow_mut().children.insert(c, cur.clone());
        }
        let res = !cur.borrow().match_flag;
        cur.borrow_mut().match_flag = true;
        if res {
            self.word_count += 1;
        }
        res
    }

    pub fn search<'a>(&self, s: &'a str) -> Vec<&'a str> {
        let mut res = Vec::new();
        let mut state = State::new(self);
        for (i, c) in s.chars().enumerate() {
            if let Some(node) = state.input(c) {
                res.push(&s[i + c.len_utf8() - node.borrow().height..=i]);
            }
        }
        res
    }
}

struct State {
    cur: Rc<RefCell<Node>>,
}

impl State {
    fn new(tree: &AhoCorasick) -> State {
        State {
            cur: Rc::clone(&tree.root),
        }
    }

    fn input(&mut self, c: char) -> Option<Rc<RefCell<Node>>> {
        let mut parent_node = Rc::clone(&self.cur);
        loop {
            if let Some(child) = parent_node.borrow().children.get(&c) {
                self.cur = Rc::clone(child);
                break;
            }
            let suffix = parent_node.borrow().suffix.clone();
            match suffix.upgrade() {
                Some(node) => parent_node = node,
                None => {
                    self.cur = parent_node;
                    break;
                }
            }
        }
        if self.cur.borrow().match_flag {
            Some(Rc::clone(&self.cur))
        } else {
            None
        }
    }
}

impl<'a, S: AsRef<str>> FromIterator<S> for AhoCorasick {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> AhoCorasick {
        let mut res = AhoCorasick::new();
        for s in iter.into_iter() {
            res.add_word(s.as_ref());
        }
        res
    }
}

struct Node {
    match_flag: bool,
    children: HashMap<char, Rc<RefCell<Node>>>,
    suffix: Weak<RefCell<Node>>,
    height: usize,
}

impl Node {
    fn new() -> Node {
        Node {
            match_flag: false,
            children: HashMap::new(),
            suffix: Weak::new(),
            height: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
        let dict = ["abc", "abcd", "xyz", "acxy", "efg", "123", "678", "6543"];
        let s = "ababcxyzacxy12678acxy6543";

        let ac_machine: AhoCorasick = dict.iter().collect();
        let res = ac_machine.search(s);
        assert_eq!(res, ["abc", "xyz", "acxy", "678", "acxy", "6543",]);
    }
}
