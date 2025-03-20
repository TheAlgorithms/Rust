use nalgebra::Matrix2;

/// Returns Some(inverse) if it exists (i.e. when gcd(a, m) == 1), else None.
fn mod_inv(a: i32, m: i32) -> Option<i32> {
    (1..m).find(|&x| (a * x) % m == 1)
}

fn check_key_validity(key: &Matrix2<i32>) -> Result<(), &'static str> {
    let det = key[(0, 0)] * key[(1, 1)] - key[(0, 1)] * key[(1, 0)];

    if det.rem_euclid(26) == 0 {
        return Err("Error: key matrix is not invertible mod 26");
    }
    Ok(())
}

fn text_to_numbers(text: &str) -> Vec<u8> {
    text.chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                Some(c.to_ascii_uppercase() as u8 - b'A')
            } else {
                None
            }
        })
        .collect()
}

fn numbers_to_text(nums: &[u8]) -> String {
    nums.iter()
        .map(|&n| ((n.rem_euclid(26)) + b'A') as char)
        .collect()
}

fn process_2x2_chunks(text_vector: Vec<u8>, key: &Matrix2<i32>) -> Vec<u8> {
    let mut result_data = Vec::new();

    for chunk in text_vector.chunks(2) {
        // if chunk is incomplete then padding with 0
        let a = *chunk.first().unwrap_or(&0) as i32;
        let b = *chunk.get(1).unwrap_or(&0) as i32;

        // matrix mult
        let x = key[(0, 0)] * a + key[(0, 1)] * b;
        let y = key[(1, 0)] * a + key[(1, 1)] * b;
        result_data.push(x.rem_euclid(26) as u8);
        result_data.push(y.rem_euclid(26) as u8);
    }

    result_data
}

pub fn encode_hill(text: &str, key: Matrix2<i32>) -> Result<String, &'static str> {
    check_key_validity(&key)?;

    let text_vector = text_to_numbers(text);
    let encrypted_data = process_2x2_chunks(text_vector, &key);
    Ok(numbers_to_text(&encrypted_data))
}

pub fn decode_hill(text: &str, key: Matrix2<i32>) -> Result<String, &'static str> {
    check_key_validity(&key)?;

    let det = key[(0, 0)] * key[(1, 1)] - key[(0, 1)] * key[(1, 0)];
    let det_mod = det.rem_euclid(26);
    let det_inv = match mod_inv(det_mod, 26) {
        Some(x) => x,
        None => return Err("Error: key matrix is not invertible mod 26"),
    };

    // compute the inverse using 2x2 formula
    let inv_key = Matrix2::new(
        (key[(1, 1)] * det_inv).rem_euclid(26),
        (-key[(0, 1)] * det_inv).rem_euclid(26),
        (-key[(1, 0)] * det_inv).rem_euclid(26),
        (key[(0, 0)] * det_inv).rem_euclid(26),
    );

    let text_vector = text_to_numbers(text);
    let decrypted_data = process_2x2_chunks(text_vector, &inv_key);
    Ok(numbers_to_text(&decrypted_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_not_invertible_should_fail() {
        let key = Matrix2::new(2, 4, 1, 2);
        assert_eq!(
            encode_hill("test", key),
            Err("Error: key matrix is not invertible mod 26")
        );
    }

    #[test]
    fn test_text_to_numbers_conversion() {
        assert_eq!(text_to_numbers("123 ABC xyz!?"), vec![0, 1, 2, 23, 24, 25]);
    }

    #[test]
    fn test_numbers_to_text_conversion() {
        assert_eq!(numbers_to_text(&[0, 1, 2, 23, 24, 25]), "ABCXYZ");
    }

    #[test]
    fn test_encoding_with_valid_key() {
        let key = Matrix2::new(3, 3, 2, 5);
        let result = encode_hill("HELP", key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decoding_with_valid_key() {
        let key = Matrix2::new(3, 3, 2, 5);
        let encoded_text = encode_hill("HELP", key).unwrap();
        let decoded_text = decode_hill(&encoded_text, key).unwrap();
        assert_eq!(decoded_text, "HELP");
    }

    #[test]
    fn test_encoding_with_padding() {
        let key = Matrix2::new(3, 3, 2, 5);
        let result = encode_hill("ABC", key);
        assert!(result.is_ok());
    }
}
