// https://en.wikipedia.org/wiki/Move-to-front_transform

pub fn move_to_front_encode(text: &str) -> Vec<u8> {
    let mut char_table: Vec<char> = (0..=255).map(|ch| ch as u8 as char).collect();
    let mut result = Vec::new();

    for ch in text.chars() {
        if let Some(position) = char_table.iter().position(|&x| x == ch) {
            result.push(position as u8);
            char_table.remove(position);
            char_table.insert(0, ch);
        }
    }

    result
}

pub fn move_to_front_decode(encoded: &[u8]) -> String {
    let mut char_table: Vec<char> = (0..=255).map(|ch| ch as u8 as char).collect();
    let mut result = String::new();

    for &pos in encoded {
        let ch = char_table[pos as usize];
        result.push(ch);
        char_table.remove(pos as usize);
        char_table.insert(0, ch);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_to_front_encode() {
        assert_eq!(move_to_front_encode(""), []);
        assert_eq!(move_to_front_encode("@"), [64]);
        assert_eq!(move_to_front_encode("aaba"), [97, 0, 98, 1]);
        assert_eq!(move_to_front_encode("aZ!"), [97, 91, 35]);
        assert_eq!(move_to_front_encode("banana"), [98, 98, 110, 1, 1, 1]);
        assert_eq!(move_to_front_encode("\0\n\t"), [0, 10, 10]);
    }

    #[test]
    fn test_move_to_front_decode() {
        assert_eq!(move_to_front_decode(&[]), "");
        assert_eq!(move_to_front_decode(&[64]), "@");
        assert_eq!(move_to_front_decode(&[97, 0, 98, 1]), "aaba");
        assert_eq!(move_to_front_decode(&[97, 91, 35]), "aZ!");
        assert_eq!(move_to_front_decode(&[98, 98, 110, 1, 1, 1]), "banana");
        assert_eq!(move_to_front_decode(&[0, 10, 10]), "\0\n\t");
    }
}
