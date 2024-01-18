use std::collections::{BTreeMap, BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum HuffmanNode {
    Internal {
        frequency: usize,
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
    Leaf {
        character: char,
        frequency: usize,
    },
}

pub fn huffman_encode(text: &str) -> (String, BTreeMap<char, String>) {
    let mut frequencies = HashMap::new();

    for char in text.chars() {
        let counter = frequencies.entry(char).or_insert(0);
        *counter += 1;
    }

    let mut heap = BinaryHeap::new();
    for (char, freq) in frequencies.iter() {
        heap.push(HuffmanNode::Leaf {
            character: *char,
            frequency: *freq,
        });
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let internal_node = HuffmanNode::Internal {
            frequency: match (&left, &right) {
                (
                    HuffmanNode::Internal { frequency: f1, .. },
                    HuffmanNode::Internal { frequency: f2, .. },
                )
                | (
                    HuffmanNode::Internal { frequency: f1, .. },
                    HuffmanNode::Leaf { frequency: f2, .. },
                )
                | (
                    HuffmanNode::Leaf { frequency: f1, .. },
                    HuffmanNode::Internal { frequency: f2, .. },
                )
                | (
                    HuffmanNode::Leaf { frequency: f1, .. },
                    HuffmanNode::Leaf { frequency: f2, .. },
                ) => f1 + f2,
            },
            left: Box::new(left),
            right: Box::new(right),
        };
        heap.push(internal_node);
    }

    let huffman_tree = heap.pop().unwrap();
    let mut huffman_codes = BTreeMap::new();
    generate_huffman_code(&huffman_tree, String::new(), &mut huffman_codes);

    let encoded_text: String = text.chars().map(|c| huffman_codes[&c].clone()).collect();

    (encoded_text, huffman_codes)
}

pub fn generate_huffman_code(tree: &HuffmanNode, code: String, codes: &mut BTreeMap<char, String>) {
    match tree {
        HuffmanNode::Leaf { character, .. } => {
            codes.insert(*character, code);
        }
        HuffmanNode::Internal { left, right, .. } => {
            let left_code = code.clone() + "0";
            generate_huffman_code(left, left_code, codes);

            let right_code = code + "1";
            generate_huffman_code(right, right_code, codes);
        }
    }
}

pub fn huffman_decode(encoded_text: &str, huffman_codes: &BTreeMap<char, String>) -> String {
    let mut decoded_text = String::new();
    let mut current_code = String::new();

    for bit in encoded_text.chars() {
        current_code.push(bit);
        if let Some((character, _)) = huffman_codes
            .iter()
            .find(|(_, code)| **code == current_code)
        {
            decoded_text.push(*character);
            current_code.clear();
        }
    }

    decoded_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_encode_decode() {
        let input_text = "abracadabra";
        let (encoded_text, huffman_codes) = huffman_encode(input_text);
        let decoded_text = huffman_decode(&encoded_text, &huffman_codes);

        assert_eq!(input_text, decoded_text);
    }
}
