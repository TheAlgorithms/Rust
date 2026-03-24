// SHA-1 (FIPS 180-4) is defined with big-endian word/length encoding.
// Clippy's `big_endian_bytes` lint would incorrectly flag every intentional
// `to_be_bytes` / `from_be_bytes` call in this file.
#![allow(clippy::big_endian_bytes)]

/// Block size in bits
const BLOCK_BITS: usize = 512;
const BLOCK_BYTES: usize = BLOCK_BITS / 8;
const BLOCK_WORDS: usize = BLOCK_BYTES / 4;

/// Digest size in bits and bytes
const DIGEST_BITS: usize = 160;
const DIGEST_BYTES: usize = DIGEST_BITS / 8;

/// Number of rounds per block
const ROUNDS: usize = 80;

/// Initial hash values (first 32 bits of the fractional parts of the square roots of the first
/// five primes)
const H_INIT: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];

/// Round constants
const K: [u32; 4] = [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6];

/// Nonlinear mixing functions for each of the four 20-round stages
fn ch(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | ((!b) & d)
}

fn parity(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

fn maj(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (b & d) | (c & d)
}

/// Selects the mixing function and round constant for a given round index
fn round_params(t: usize) -> (fn(u32, u32, u32) -> u32, u32) {
    match t {
        0..=19 => (ch, K[0]),
        20..=39 => (parity, K[1]),
        40..=59 => (maj, K[2]),
        60..=79 => (parity, K[3]),
        _ => unreachable!(),
    }
}

/// Pads the message to a multiple of 512 bits.
///
/// SHA-1 padding appends a single `1` bit, enough `0` bits, and finally the original
/// message length as a 64-bit big-endian integer, such that the total length is
/// congruent to 0 mod 512.
fn pad(message: &[u8]) -> Vec<u8> {
    let bit_len = (message.len() as u64).wrapping_mul(8);

    let mut padded = message.to_vec();
    padded.push(0x80); // append the '1' bit followed by seven '0' bits

    // Append zero bytes until length ≡ 56 (mod 64)
    while padded.len() % BLOCK_BYTES != 56 {
        padded.push(0x00);
    }

    // Append original length as 64-bit big-endian
    padded.extend_from_slice(&bit_len.to_be_bytes());

    debug_assert!(padded.len().is_multiple_of(BLOCK_BYTES));
    padded
}

/// Parses a 64-byte block into sixteen 32-bit big-endian words
fn parse_block(block: &[u8]) -> [u32; BLOCK_WORDS] {
    debug_assert_eq!(block.len(), BLOCK_BYTES);

    let mut words = [0u32; BLOCK_WORDS];
    for (i, word) in words.iter_mut().enumerate() {
        *word = u32::from_be_bytes(block[i * 4..i * 4 + 4].try_into().unwrap());
    }
    words
}

/// Expands sixteen message words into eighty scheduled words using the message schedule
fn schedule(m: [u32; BLOCK_WORDS]) -> [u32; ROUNDS] {
    let mut w = [0u32; ROUNDS];
    w[..BLOCK_WORDS].copy_from_slice(&m);

    for t in BLOCK_WORDS..ROUNDS {
        w[t] = (w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]).rotate_left(1);
    }
    w
}

/// Processes a single 512-bit block, updating the running hash state in place
fn compress(state: &mut [u32; 5], block: &[u8]) {
    let w = schedule(parse_block(block));

    let [mut a, mut b, mut c, mut d, mut e] = *state;

    for (t, &w_t) in w.iter().enumerate() {
        let (f, k) = round_params(t);
        let temp = a
            .rotate_left(5)
            .wrapping_add(f(b, c, d))
            .wrapping_add(e)
            .wrapping_add(k)
            .wrapping_add(w_t);
        e = d;
        d = c;
        c = b.rotate_left(30);
        b = a;
        a = temp;
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}

/// Computes the SHA-1 digest of the given byte slice, returning a 20-byte array
pub fn sha1(message: &[u8]) -> [u8; DIGEST_BYTES] {
    let padded = pad(message);
    let mut state = H_INIT;

    for block in padded.chunks(BLOCK_BYTES) {
        compress(&mut state, block);
    }

    // Serialise the five 32-bit words into twenty bytes (big-endian)
    let mut digest = [0u8; DIGEST_BYTES];
    for (i, &word) in state.iter().enumerate() {
        digest[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Convenience macro that generates a named test, hashing `$input` and comparing the
    /// result byte-for-byte against `$expected`.
    macro_rules! sha1_test {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let digest = sha1($input);
                let expected: [u8; DIGEST_BYTES] = $expected;
                assert_eq!(digest, expected);
            }
        };
    }

    // ── NIST FIPS 180-4 / RFC 3174 test vectors ──────────────────────────────

    // SHA1("") = da39a3ee 5e6b4b0d 3255bfef 95601890 afd80709
    sha1_test!(
        sha1_empty,
        b"",
        [
            0xda, 0x39, 0xa3, 0xee, 0x5e, 0x6b, 0x4b, 0x0d, 0x32, 0x55, 0xbf, 0xef, 0x95, 0x60,
            0x18, 0x90, 0xaf, 0xd8, 0x07, 0x09,
        ]
    );

    // SHA1("abc") = a9993e36 4706816a ba3e2571 7850c26c 9cd0d89d
    sha1_test!(
        sha1_abc,
        b"abc",
        [
            0xa9, 0x99, 0x3e, 0x36, 0x47, 0x06, 0x81, 0x6a, 0xba, 0x3e, 0x25, 0x71, 0x78, 0x50,
            0xc2, 0x6c, 0x9c, 0xd0, 0xd8, 0x9d,
        ]
    );

    // SHA1("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
    //   = 84983e44 1c3bd26e baae4aa1 f95129e5 e54670f1
    sha1_test!(
        sha1_448_bit,
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        [
            0x84, 0x98, 0x3e, 0x44, 0x1c, 0x3b, 0xd2, 0x6e, 0xba, 0xae, 0x4a, 0xa1, 0xf9, 0x51,
            0x29, 0xe5, 0xe5, 0x46, 0x70, 0xf1,
        ]
    );

    // SHA1("abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu")
    //   = a49b2446 a02c645b f419f995 b6709125 3a04a259
    sha1_test!(
        sha1_896_bit,
        b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
        [
            0xa4, 0x9b, 0x24, 0x46, 0xa0, 0x2c, 0x64, 0x5b, 0xf4, 0x19, 0xf9, 0x95, 0xb6, 0x70,
            0x91, 0x25, 0x3a, 0x04, 0xa2, 0x59,
        ]
    );

    // SHA1("a" × 1 000 000) = 34aa973c d4c4daa4 f61eeb2b dbad2731 6534016f
    // Verifies that the sponge-like multi-block path is exercised correctly.
    #[test]
    fn sha1_million_a() {
        let input = vec![b'a'; 1_000_000];
        let digest = sha1(&input);
        let expected: [u8; DIGEST_BYTES] = [
            0x34, 0xaa, 0x97, 0x3c, 0xd4, 0xc4, 0xda, 0xa4, 0xf6, 0x1e, 0xeb, 0x2b, 0xdb, 0xad,
            0x27, 0x31, 0x65, 0x34, 0x01, 0x6f,
        ];
        assert_eq!(digest, expected);
    }
}
