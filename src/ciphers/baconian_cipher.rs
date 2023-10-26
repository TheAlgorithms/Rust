// Author : cyrixninja
//Program to encode and decode Baconian or Bacon's Cipher
//Wikipedia reference : https://en.wikipedia.org/wiki/Bacon%27s_cipher
// Bacon's cipher or the Baconian cipher is a method of steganographic message encoding devised by Francis Bacon in 1605.
// A message is concealed in the presentation of text, rather than its content. Bacon cipher is categorized as both a substitution cipher (in plain code) and a concealment cipher (using the two typefaces).

// Encode Baconian Cipher
pub fn baconian_encode(message: &str) -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let baconian = [
        "AAAAA", "AAAAB", "AAABA", "AAABB", "AABAA", "AABAB", "AABBA", "AABBB", "ABAAA", "ABAAB",
        "ABABA", "ABABB", "ABBAA", "ABBAB", "ABBBA", "ABBBB", "BAAAA", "BAAAB", "BAABA", "BAABB",
        "BABAA", "BABAB", "BABBA", "BABBB",
    ];

    message
        .chars()
        .map(|c| {
            if let Some(index) = alphabet.find(c.to_ascii_uppercase()) {
                baconian[index].to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

// Decode Baconian Cipher
pub fn baconian_decode(encoded: &str) -> String {
    let baconian = [
        "AAAAA", "AAAAB", "AAABA", "AAABB", "AABAA", "AABAB", "AABBA", "AABBB", "ABAAA", "ABAAB",
        "ABABA", "ABABB", "ABBAA", "ABBAB", "ABBBA", "ABBBB", "BAAAA", "BAAAB", "BAABA", "BAABB",
        "BABAA", "BABAB", "BABBA", "BABBB",
    ];
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    encoded
        .as_bytes()
        .chunks(5)
        .map(|chunk| {
            if let Some(index) = baconian
                .iter()
                .position(|&x| x == String::from_utf8_lossy(chunk))
            {
                alphabet.chars().nth(index).unwrap()
            } else {
                ' '
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baconian_encoding() {
        let message = "HELLO";
        let encoded = baconian_encode(message);
        assert_eq!(encoded, "AABBBAABAAABABBABABBABBBA");
    }

    #[test]
    fn test_baconian_decoding() {
        let message = "AABBBAABAAABABBABABBABBBA";
        let decoded = baconian_decode(message);
        assert_eq!(decoded, "HELLO");
    }
}
