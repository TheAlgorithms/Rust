use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct HuffmanValue {
    // For the `value` to overflow, the sum of frequencies should be bigger
    // than u64. So we should be safe here
    /// The encoded value
    pub value: u64,
    /// number of bits used (up to 64)
    pub bits: u32,
}

pub struct HuffmanNode<T> {
    pub left: Option<Box<HuffmanNode<T>>>,
    pub right: Option<Box<HuffmanNode<T>>>,
    pub symbol: Option<T>,
    pub frequency: u64,
}

impl<T> PartialEq for HuffmanNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl<T> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for HuffmanNode<T> {}

impl<T> Ord for HuffmanNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

impl<T: Clone + Copy + Ord> HuffmanNode<T> {
    /// Turn the tree into the map that can be used in encoding
    pub fn get_alphabet(
        height: u32,
        path: u64,
        node: &HuffmanNode<T>,
        map: &mut BTreeMap<T, HuffmanValue>,
    ) {
        match node.symbol {
            Some(s) => {
                map.insert(
                    s,
                    HuffmanValue {
                        value: path,
                        bits: height,
                    },
                );
            }
            None => {
                Self::get_alphabet(height + 1, path, node.left.as_ref().unwrap(), map);
                Self::get_alphabet(
                    height + 1,
                    path | (1 << height),
                    node.right.as_ref().unwrap(),
                    map,
                );
            }
        }
    }
}

pub struct HuffmanDictionary<T> {
    pub alphabet: BTreeMap<T, HuffmanValue>,
    pub root: HuffmanNode<T>,
}

impl<T: Clone + Copy + Ord> HuffmanDictionary<T> {
    /// Creates a new Huffman dictionary from alphabet symbols and their frequencies.
    ///
    /// Returns `None` if the alphabet is empty.
    ///
    /// # Arguments
    /// * `alphabet` - A slice of tuples containing symbols and their frequencies
    ///
    /// # Example
    /// ```
    /// # use the_algorithms_rust::general::HuffmanDictionary;
    /// let freq = vec![('a', 5), ('b', 2), ('c', 1)];
    /// let dict = HuffmanDictionary::new(&freq).unwrap();
    ///
    pub fn new(alphabet: &[(T, u64)]) -> Option<Self> {
        if alphabet.is_empty() {
            return None;
        }

        let mut alph: BTreeMap<T, HuffmanValue> = BTreeMap::new();

        // Special case: single symbol
        if alphabet.len() == 1 {
            let (symbol, _freq) = alphabet[0];
            alph.insert(
                symbol,
                HuffmanValue {
                    value: 0,
                    bits: 1, // Must use at least 1 bit per symbol
                },
            );

            let root = HuffmanNode {
                left: None,
                right: None,
                symbol: Some(symbol),
                frequency: alphabet[0].1,
            };

            return Some(HuffmanDictionary {
                alphabet: alph,
                root,
            });
        }

        let mut queue: BinaryHeap<HuffmanNode<T>> = BinaryHeap::new();
        for (symbol, freq) in alphabet.iter() {
            queue.push(HuffmanNode {
                left: None,
                right: None,
                symbol: Some(*symbol),
                frequency: *freq,
            });
        }
        for _ in 1..alphabet.len() {
            let left = queue.pop().unwrap();
            let right = queue.pop().unwrap();
            let sm_freq = left.frequency + right.frequency;
            queue.push(HuffmanNode {
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
                symbol: None,
                frequency: sm_freq,
            });
        }
        if let Some(root) = queue.pop() {
            HuffmanNode::get_alphabet(0, 0, &root, &mut alph);
            Some(HuffmanDictionary {
                alphabet: alph,
                root,
            })
        } else {
            None
        }
    }
    pub fn encode(&self, data: &[T]) -> HuffmanEncoding {
        let mut result = HuffmanEncoding::new();
        data.iter()
            .for_each(|value| result.add_data(self.alphabet[value]));
        result
    }
}
pub struct HuffmanEncoding {
    pub num_bits: u64,
    pub data: Vec<u64>,
}

impl Default for HuffmanEncoding {
    fn default() -> Self {
        Self::new()
    }
}

impl HuffmanEncoding {
    pub fn new() -> Self {
        HuffmanEncoding {
            num_bits: 0,
            data: vec![0],
        }
    }
    #[inline]
    pub fn add_data(&mut self, data: HuffmanValue) {
        let shift = (self.num_bits & 63) as u32;
        let val = data.value;
        *self.data.last_mut().unwrap() |= val.wrapping_shl(shift);
        if (shift + data.bits) >= 64 {
            self.data.push(val.wrapping_shr(64 - shift));
        }
        self.num_bits += data.bits as u64;
    }

    #[inline]
    fn get_bit(&self, pos: u64) -> bool {
        (self.data[(pos >> 6) as usize] & (1 << (pos & 63))) != 0
    }

    /// In case the encoding is invalid, `None` is returned
    pub fn decode<T: Clone + Copy + Ord>(&self, dict: &HuffmanDictionary<T>) -> Option<Vec<T>> {
        // Handle empty encoding
        if self.num_bits == 0 {
            return Some(vec![]);
        }

        // Special case: single symbol in dictionary
        if dict.alphabet.len() == 1 {
            //all bits represent the same symbol
            let symbol = dict.alphabet.keys().next()?;
            let result = vec![*symbol; self.num_bits as usize];
            return Some(result);
        }

        // Normal case: multiple symbols
        let mut state = &dict.root;
        let mut result: Vec<T> = vec![];

        for i in 0..self.num_bits {
            if let Some(symbol) = state.symbol {
                result.push(symbol);
                state = &dict.root;
            }
            state = if self.get_bit(i) {
                state.right.as_ref()?
            } else {
                state.left.as_ref()?
            }
        }

        // Check if we ended on a symbol
        if self.num_bits > 0 {
            result.push(state.symbol?);
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_frequency(bytes: &[u8]) -> Vec<(u8, u64)> {
        let mut cnts: Vec<u64> = vec![0; 256];
        bytes.iter().for_each(|&b| cnts[b as usize] += 1);
        let mut result = vec![];
        cnts.iter()
            .enumerate()
            .filter(|(_, &v)| v > 0)
            .for_each(|(b, &cnt)| result.push((b as u8, cnt)));
        result
    }

    #[test]
    fn empty_text() {
        let text = "";
        let bytes = text.as_bytes();
        let freq = get_frequency(bytes);
        let dict = HuffmanDictionary::new(&freq);
        assert!(dict.is_none());
    }

    #[test]
    fn one_symbol_text() {
        let text = "aaaa";
        let bytes = text.as_bytes();
        let freq = get_frequency(bytes);
        let dict = HuffmanDictionary::new(&freq).unwrap();
        let encoded = dict.encode(bytes);
        assert_eq!(encoded.num_bits, 4);
        let decoded = encoded.decode(&dict).unwrap();
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn test_decode_empty_encoding_struct() {
        // Create a minimal but VALID HuffmanDictionary.
        // This is required because decode() expects a dictionary, even though
        // the content of the dictionary doesn't matter when num_bits == 0.
        let freq = vec![('a' as u8, 1)];
        let dict = HuffmanDictionary::new(&freq).unwrap();

        // Manually create the target state: an encoding with 0 bits.
        let empty_encoding = HuffmanEncoding {
            data: vec![],
            num_bits: 0,
        };

        let result = empty_encoding.decode(&dict);

        assert_eq!(result, Some(vec![]));
    }

    #[test]
    fn minimal_decode_end_check() {
        let freq = vec![(b'a' as u8, 1), (b'b' as u8, 1)];
        let bytes = b"ab";

        let dict = HuffmanDictionary::new(&freq).unwrap();
        let encoded = dict.encode(bytes);

        // This decode will go through the main loop and hit the final 'if self.num_bits > 0' check.
        let decoded = encoded.decode(&dict).unwrap();

        assert_eq!(decoded, bytes);
    }

    #[test]
    fn small_text() {
        let text = "Hello world";
        let bytes = text.as_bytes();
        let freq = get_frequency(bytes);
        let dict = HuffmanDictionary::new(&freq).unwrap();
        let encoded = dict.encode(bytes);
        assert_eq!(encoded.num_bits, 32);
        let decoded = encoded.decode(&dict).unwrap();
        assert_eq!(decoded, bytes);
    }
    #[test]
    fn lorem_ipsum() {
        let text = concat!(
            "The quick brown fox jumped over the lazy dog.",
            "Lorem ipsum dolor sit amet, consectetur ",
            "adipiscing elit, sed do eiusmod tempor incididunt ut labore et ",
            "dolore magna aliqua. Facilisis magna etiam tempor orci. Nullam ",
            "non nisi est sit amet facilisis magna. Commodo nulla facilisi ",
            "nullam vehicula. Interdum posuere lorem ipsum dolor. Elit eget ",
            "gravida cum sociis natoque penatibus. Dictum sit amet justo donec ",
            "enim. Tempor commodo ullamcorper a lacus vestibulum sed. Nisl ",
            "suscipit adipiscing bibendum est ultricies. Sit amet aliquam id ",
            "diam maecenas ultricies."
        );
        let bytes = text.as_bytes();
        let freq = get_frequency(bytes);
        let dict = HuffmanDictionary::new(&freq).unwrap();
        let encoded = dict.encode(bytes);
        assert_eq!(encoded.num_bits, 2372);
        let decoded = encoded.decode(&dict).unwrap();
        assert_eq!(decoded, bytes);

        let text = "The dictionary should work on other texts too";
        let bytes = text.as_bytes();
        let encoded = dict.encode(bytes);
        assert_eq!(encoded.num_bits, 215);
        let decoded = encoded.decode(&dict).unwrap();
        assert_eq!(decoded, bytes);
    }
}
