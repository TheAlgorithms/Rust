// In computer science, a suffix tree (also called PAT tree or, in an earlier form, position tree)
// is a compressed trie containing all the suffixes of the given text as their keys and positions
// in the text as their values. Suffix trees allow particularly fast implementations of many
// important string operations. Source: https://en.wikipedia.org/wiki/Suffix_tree

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub sub: String,    // substring of input string
    pub ch: Vec<usize>, // vector of child nodes
}

impl Node {
    fn new(sub: String, children: Vec<usize>) -> Self {
        Node {
            sub,
            ch: children.to_vec(),
        }
    }
    pub fn empty() -> Self {
        Node {
            sub: "".to_string(),
            ch: vec![],
        }
    }
}

pub struct SuffixTree {
    pub nodes: Vec<Node>,
}

impl SuffixTree {
    pub fn new(s: &str) -> Self {
        let mut suf_tree = SuffixTree {
            nodes: vec![Node::empty()],
        };
        for i in 0..s.len() {
            let (_, substr) = s.split_at(i);
            suf_tree.add_suffix(substr);
        }
        suf_tree
    }
    fn add_suffix(&mut self, suf: &str) {
        let mut n = 0;
        let mut i = 0;
        while i < suf.len() {
            let b = suf.chars().nth(i);
            let mut x2 = 0;
            let mut n2: usize;
            loop {
                let children = &self.nodes[n].ch;
                if children.len() == x2 {
                    n2 = self.nodes.len();
                    self.nodes.push(Node::new(
                        {
                            let (_, sub) = suf.split_at(i);
                            sub.to_string()
                        },
                        vec![],
                    ));
                    self.nodes[n].ch.push(n2);
                    return;
                }
                n2 = children[x2];
                if self.nodes[n2].sub.chars().next() == b {
                    break;
                }
                x2 += 1;
            }
            let sub2 = self.nodes[n2].sub.clone();
            let mut j = 0;
            while j < sub2.len() {
                if suf.chars().nth(i + j) != sub2.chars().nth(j) {
                    let n3 = n2;
                    n2 = self.nodes.len();
                    self.nodes.push(Node::new(
                        {
                            let (sub, _) = sub2.split_at(j);
                            sub.to_string()
                        },
                        vec![n3],
                    ));
                    let (_, temp_sub) = sub2.split_at(j);
                    self.nodes[n3].sub = temp_sub.to_string();
                    self.nodes[n].ch[x2] = n2;
                    break;
                }
                j += 1;
            }
            i += j;
            n = n2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_tree() {
        let suf_tree = SuffixTree::new("banana$");
        assert_eq!(
            suf_tree.nodes,
            vec![
                Node {
                    sub: "".to_string(),
                    ch: vec![1, 8, 6, 10]
                },
                Node {
                    sub: "banana$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na".to_string(),
                    ch: vec![2, 5]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na".to_string(),
                    ch: vec![3, 7]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "a".to_string(),
                    ch: vec![4, 9]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                }
            ]
        );
    }
}
