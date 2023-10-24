// Author : cyrixninja
// Octal to Binary Converter : Converts Octal to Binary
// Wikipedia References  : 1. https://en.wikipedia.org/wiki/Octal
//                         2. https://en.wikipedia.org/wiki/Binary_number

pub fn octal_to_binary(octal_str: &str) -> Result<String, &'static str> {
    let octal_str = octal_str.trim();

    if octal_str.is_empty() {
        return Err("Empty");
    }

    if !octal_str.chars().all(|c| ('0'..'7').contains(&c)) {
        return Err("Non-octal Value");
    }

    // Convert octal to binary
    let binary = octal_str
        .chars()
        .map(|c| match c {
            '0' => "000",
            '1' => "001",
            '2' => "010",
            '3' => "011",
            '4' => "100",
            '5' => "101",
            '6' => "110",
            '7' => "111",
            _ => unreachable!(),
        })
        .collect::<String>();

    Ok(binary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = Err("Empty");
        assert_eq!(octal_to_binary(input), expected);
    }

    #[test]
    fn test_invalid_octal() {
        let input = "89";
        let expected = Err("Non-octal Value");
        assert_eq!(octal_to_binary(input), expected);
    }

    #[test]
    fn test_valid_octal() {
        let input = "123";
        let expected = Ok("001010011".to_string());
        assert_eq!(octal_to_binary(input), expected);
    }
}
