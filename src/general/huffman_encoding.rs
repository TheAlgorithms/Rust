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
    /// The list of alphabet symbols and their respective frequency should
    /// be given as input
    pub fn new(alphabet: &[(T, u64)]) -> Self {
        let mut alph: BTreeMap<T, HuffmanValue> = BTreeMap::new();
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
        let root = queue.pop().unwrap();
        HuffmanNode::get_alphabet(0, 0, &root, &mut alph);
        HuffmanDictionary {
            alphabet: alph,
            root,
        }
    }
    pub fn encode(&self, data: &[T]) -> HuffmanEncoding {
        let mut result = HuffmanEncoding::new();
        data.iter()
            .for_each(|value| result.add_data(*self.alphabet.get(value).unwrap()));
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
    fn get_bit(&self, pos: u64) -> bool {
        (self.data[(pos >> 6) as usize] & (1 << (pos & 63))) != 0
    }
    /// In case the encoding is invalid, `None` is returned
    pub fn decode<T: Clone + Copy + Ord>(&self, dict: &HuffmanDictionary<T>) -> Option<Vec<T>> {
        let mut state = &dict.root;
        let mut result: Vec<T> = vec![];
        for i in 0..self.num_bits {
            if state.symbol.is_some() {
                result.push(state.symbol.unwrap());
                state = &dict.root;
            }
            match self.get_bit(i) {
                false => state = state.left.as_ref().unwrap(),
                true => state = state.right.as_ref().unwrap(),
            }
        }
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
    fn small_text() {
        let text = "Hello world";
        let bytes = text.as_bytes();
        let freq = get_frequency(bytes);
        let dict = HuffmanDictionary::new(&freq);
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
        let dict = HuffmanDictionary::new(&freq);
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
