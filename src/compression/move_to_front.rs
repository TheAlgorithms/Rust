// https://en.wikipedia.org/wiki/Move-to-front_transform

fn blank_char_table() -> Vec<char> {
    (0..=255).map(|ch| ch as u8 as char).collect()
}

pub fn move_to_front_encode(text: &str) -> Vec<u8> {
    let mut char_table = blank_char_table();
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
    let mut char_table = blank_char_table();
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

    macro_rules! test_mtf {
        ($($name:ident: ($text:expr, $encoded:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(move_to_front_encode($text), $encoded);
                    assert_eq!(move_to_front_decode(&$encoded), $text);
                }
            )*
        }
    }

    test_mtf! {
        empty: ("", vec![]),
        single_char: ("@", vec![64]),
        repeated_chars: ("aaba", vec![97, 0, 98, 1]),
        mixed_chars: ("aZ!", vec![97, 91, 35]),
        word: ("banana", vec![98, 98, 110, 1, 1, 1]),
        special_chars: ("\0\n\t", vec![0, 10, 10]),
    }
}
