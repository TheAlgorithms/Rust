use std::fmt::Write;

// MD5 hash function implementation
// Reference: https://www.ietf.org/rfc/rfc1321.txt
//
// MD5 produces a 128-bit (16-byte) hash value.
// Note: MD5 is cryptographically broken and should NOT be used for security
// purposes. It remains useful for checksums and non-security applications.

/// Per-round shift amounts (RFC 1321, §3.4)
const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, // Round 1
    5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, // Round 2
    4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, // Round 3
    6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, // Round 4
];

/// Precomputed table of abs(sin(i+1)) * 2^32 (RFC 1321, §3.4)
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

/// Initial hash state (RFC 1321, §3.3) — "magic" little-endian constants
const INIT_STATE: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

/// Computes the MD5 hash of the given byte slice.
/// Returns a 16-byte array representing the 128-bit digest.
pub fn md5(input: &[u8]) -> [u8; 16] {
    let mut state = INIT_STATE;

    // --- Pre-processing: padding ---
    // Append bit '1' (0x80 byte), then zeros, then 64-bit little-endian
    // message length in bits, so total length ≡ 448 (mod 512) bits.
    let bit_len = (input.len() as u64).wrapping_mul(8);
    let mut msg = input.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());

    // --- Processing: 512-bit (64-byte) chunks ---
    for chunk in msg.chunks_exact(64) {
        // Break chunk into 16 little-endian 32-bit words
        let mut m = [0u32; 16];
        for (i, word) in m.iter_mut().enumerate() {
            let offset = i * 4;
            *word = u32::from_le_bytes(chunk[offset..offset + 4].try_into().unwrap());
        }

        let [mut a, mut b, mut c, mut d] = state;

        for i in 0..64u32 {
            let (f, g) = match i {
                0..=15 => ((b & c) | (!b & d), i),
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };

            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                (a.wrapping_add(f)
                    .wrapping_add(K[i as usize])
                    .wrapping_add(m[g as usize]))
                .rotate_left(S[i as usize]),
            );
            a = temp;
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
    }

    // --- Produce final digest (little-endian word order) ---
    let mut digest = [0u8; 16];
    for (i, &word) in state.iter().enumerate() {
        digest[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }
    digest
}

/// Convenience helper: returns the MD5 digest as a lowercase hex string.
pub fn md5_hex(input: &[u8]) -> String {
    md5(input)
        .iter()
        .fold(String::with_capacity(32), |mut s, b| {
            write!(s, "{b:02x}").unwrap();
            s
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // All expected values from the RFC 1321 test suite and NIST vectors.

    #[test]
    fn test_empty_string() {
        assert_eq!(md5_hex(b""), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_abc() {
        assert_eq!(md5_hex(b"abc"), "900150983cd24fb0d6963f7d28e17f72");
    }

    #[test]
    fn test_rfc_message() {
        assert_eq!(
            md5_hex(b"message digest"),
            "f96b697d7cb7938d525a2f31aaf161d0"
        );
    }

    #[test]
    fn test_alphabet() {
        assert_eq!(
            md5_hex(b"abcdefghijklmnopqrstuvwxyz"),
            "c3fcd3d76192e4007dfb496cca67e13b"
        );
    }

    #[test]
    fn test_alphanumeric() {
        assert_eq!(
            md5_hex(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            "d174ab98d277d9f5a5611c2c9f419d9f"
        );
    }

    #[test]
    fn test_digits_repeated() {
        assert_eq!(
            md5_hex(
                b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"
            ),
            "57edf4a22be3c955ac49da2e2107b67a"
        );
    }

    #[test]
    fn test_single_char() {
        assert_eq!(md5_hex(b"a"), "0cc175b9c0f1b6a831c399e269772661");
    }

    #[test]
    fn test_returns_16_bytes() {
        assert_eq!(md5(b"hello").len(), 16);
    }

    #[test]
    fn test_deterministic() {
        assert_eq!(md5(b"rust"), md5(b"rust"));
    }

    #[test]
    fn test_different_inputs_differ() {
        assert_ne!(md5(b"foo"), md5(b"bar"));
    }
}
