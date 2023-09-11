pub fn burrows_wheeler_transform(input: &str) -> (String, usize) {
    let len = input.len();

    let mut table = Vec::<String>::with_capacity(len);
    for i in 0..len {
        table.push(input[i..].to_owned() + &input[..i]);
    }
    table.sort_by_key(|a| a.to_lowercase());

    let mut encoded = String::new();
    let mut index: usize = 0;
    for (i, item) in table.iter().enumerate().take(len) {
        encoded.push(item.chars().last().unwrap());
        if item.eq(&input) {
            index = i;
        }
    }

    (encoded, index)
}

pub fn inv_burrows_wheeler_transform<T: AsRef<str>>(input: (T, usize)) -> String {
    let len = input.0.as_ref().len();
    let mut table = Vec::<(usize, char)>::with_capacity(len);
    for i in 0..len {
        table.push((i, input.0.as_ref().chars().nth(i).unwrap()));
    }

    table.sort_by(|a, b| a.1.cmp(&b.1));

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
}
