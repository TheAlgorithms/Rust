//! Burrows-Wheeler transform.
//!
//! The transform sorts every rotation of the input and emits the last character
//! of each. Inversion relies on that rotation order being *the same total order*
//! the inverse uses to sort the encoded characters, so both directions here order
//! by `char` (equivalently, by code point).
//!
//! Wikipedia reference: <https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform>

/// Returns the transformed string together with the row index of the original input.
pub fn burrows_wheeler_transform(input: &str) -> (String, usize) {
    // Rotate over `char`s: slicing `&str` by an arbitrary index splits multi-byte
    // characters and panics.
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    let mut table = Vec::<String>::with_capacity(len);
    for i in 0..len {
        table.push(chars[i..].iter().chain(&chars[..i]).collect());
    }
    table.sort();

    let mut encoded = String::new();
    let mut index: usize = 0;
    for (i, item) in table.iter().enumerate().take(len) {
        encoded.push(item.chars().last().unwrap());
        if item == input {
            index = i;
        }
    }

    (encoded, index)
}

/// Reconstructs the original string from a transform and its row index.
pub fn inv_burrows_wheeler_transform<T: AsRef<str>>(input: (T, usize)) -> String {
    let chars: Vec<char> = input.0.as_ref().chars().collect();
    let len = chars.len();

    // `sort_by_key` is stable, which is what keeps equal characters in their
    // original relative order — the property the reconstruction walk depends on.
    let mut table: Vec<(usize, char)> = chars.into_iter().enumerate().collect();
    table.sort_by_key(|a| a.1);

    let mut decoded = String::new();
    let mut idx = input.1;
    for _ in 0..len {
        decoded.push(table[idx].1);
        idx = table[idx].0;
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //Ensure function stand-alone legitimacy
    fn stand_alone_function() {
        assert_eq!(
            burrows_wheeler_transform("CARROT"),
            ("CTRRAO".to_owned(), 1usize)
        );
        assert_eq!(inv_burrows_wheeler_transform(("CTRRAO", 1usize)), "CARROT");
        assert_eq!(
            burrows_wheeler_transform("THEALGORITHMS"),
            ("EHLTTRAHGOMSI".to_owned(), 11usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform(("EHLTTRAHGOMSI".to_string(), 11usize)),
            "THEALGORITHMS"
        );
        assert_eq!(
            burrows_wheeler_transform("!.!.!??.=::"),
            (":..!!?:=.?!".to_owned(), 0usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform((":..!!?:=.?!", 0usize)),
            "!.!.!??.=::"
        );
    }
    #[test]
    fn basic_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("CARROT")),
            "CARROT"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("TOMATO")),
            "TOMATO"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THISISATEST")),
            "THISISATEST"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THEALGORITHMS")),
            "THEALGORITHMS"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("RUST")),
            "RUST"
        );
    }

    #[test]
    fn special_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!.!.!??.=::")),
            "!.!.!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!{}{}(((&&%%!??.=::")),
            "!{}{}(((&&%%!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("//&$[]")),
            "//&$[]"
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("")),
            ""
        );
    }

    /// Regression: the forward transform sorted rotations case-insensitively
    /// while the inverse sorted characters by code point, so any string mixing
    /// cases round-tripped to garbage ("Hello" came back as "elloe").
    #[test]
    fn mixed_case() {
        for text in [
            "Hello",
            "Mississippi",
            "AaAa",
            "Test",
            "aAbB",
            "Rust Lang",
            "The Algorithms",
        ] {
            assert_eq!(
                inv_burrows_wheeler_transform(burrows_wheeler_transform(text)),
                text
            );
        }
    }

    /// Regression: rotations were built by slicing `&str` at byte offsets, which
    /// panics as soon as an index lands inside a multi-byte character.
    #[test]
    fn unicode() {
        for text in [
            "café au lait",
            "日本語のテキスト",
            "naïve",
            "ábcábc",
            "🎉party🎉",
            "Ünïcödé Mïx",
        ] {
            assert_eq!(
                inv_burrows_wheeler_transform(burrows_wheeler_transform(text)),
                text
            );
        }
    }

    #[test]
    fn single_character() {
        assert_eq!(burrows_wheeler_transform("a"), ("a".to_owned(), 0));
        assert_eq!(inv_burrows_wheeler_transform(("a", 0usize)), "a");
    }

    #[test]
    fn repeated_characters() {
        for text in ["aaaa", "abab", "aaaab"] {
            assert_eq!(
                inv_burrows_wheeler_transform(burrows_wheeler_transform(text)),
                text
            );
        }
    }
}
