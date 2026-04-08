//! SHA-2 (Secure Hash Algorithm 2) family of cryptographic hash functions.
//!
//! Designed by the NSA and published by NIST in 2001 (FIPS PUB 180-4).
//! Built on the Merkle–Damgård construction with a Davies–Meyer compression
//! function. The family includes six variants differentiated by digest size
//! and internal word width:
//!
//! | Function    | Word | Rounds | Digest |
//! |-------------|------|--------|--------|
//! | SHA-224     |  32  |   64   |  224   |
//! | SHA-256     |  32  |   64   |  256   |
//! | SHA-384     |  64  |   80   |  384   |
//! | SHA-512     |  64  |   80   |  512   |
//! | SHA-512/224 |  64  |   80   |  224   |
//! | SHA-512/256 |  64  |   80   |  256   |
//!
//! Reference: <https://doi.org/10.6028/NIST.FIPS.180-4>

// SHA-2 is defined by FIPS 180-4 to use big-endian byte order throughout.
// The `clippy::big_endian_bytes` lint (from the `restriction` group) flags all
// big-endian conversions regardless of correctness; we suppress it here because
// switching to little-endian would produce wrong digests.
#![allow(clippy::big_endian_bytes)]

// ── SHA-256 / SHA-224 ────────────────────────────────────────────────────────

/// Round constants for SHA-256 / SHA-224.
///
/// First 32 bits of the fractional parts of the cube roots of the first
/// 64 prime numbers.
#[rustfmt::skip]
pub const K32: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Initial hash values for **SHA-256**.
///
/// First 32 bits of the fractional parts of the square roots of the first
/// 8 prime numbers.
#[rustfmt::skip]
pub const H256_INIT: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// Initial hash values for **SHA-224**.
#[rustfmt::skip]
const H224_INIT: [u32; 8] = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939,
    0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
];

fn sha256_pad(msg: &[u8]) -> Vec<u8> {
    let len_bits: u64 = (msg.len() as u64)
        .checked_mul(8)
        .expect("message too long for SHA-256");
    let mut padded = msg.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0x00);
    }
    padded.extend_from_slice(&len_bits.to_be_bytes());
    padded
}

fn sha256_compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    for i in 0..16 {
        w[i] = u32::from_be_bytes(block[4 * i..4 * i + 4].try_into().unwrap());
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..64 {
        let sigma1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(sigma1)
            .wrapping_add(ch)
            .wrapping_add(K32[i])
            .wrapping_add(w[i]);
        let sigma0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = sigma0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

fn sha256_hash(msg: &[u8], mut state: [u32; 8]) -> [u32; 8] {
    let padded = sha256_pad(msg);
    for chunk in padded.chunks_exact(64) {
        sha256_compress(&mut state, chunk.try_into().unwrap());
    }
    state
}

/// Computes the **SHA-256** digest of `msg`.
pub fn sha256(msg: &[u8]) -> [u8; 32] {
    let state = sha256_hash(msg, H256_INIT);
    let mut out = [0u8; 32];
    for (i, word) in state.iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// Computes the **SHA-224** digest of `msg`.
pub fn sha224(msg: &[u8]) -> [u8; 28] {
    let state = sha256_hash(msg, H224_INIT);
    let mut out = [0u8; 28];
    for (i, word) in state[..7].iter().enumerate() {
        out[4 * i..4 * i + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

// ── SHA-512 / SHA-384 / SHA-512/224 / SHA-512/256 ───────────────────────────

/// Round constants for SHA-512 / SHA-384 / SHA-512/t.
///
/// First 64 bits of the fractional parts of the cube roots of the first
/// 80 prime numbers.
#[rustfmt::skip]
const K64: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
    0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
    0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
    0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
    0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
    0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
    0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
    0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];

#[rustfmt::skip]
const H512_INIT: [u64; 8] = [
    0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
    0x510e527fade682d1, 0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
];

#[rustfmt::skip]
const H384_INIT: [u64; 8] = [
    0xcbbb9d5dc1059ed8, 0x629a292a367cd507,
    0x9159015a3070dd17, 0x152fecd8f70e5939,
    0x67332667ffc00b31, 0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7, 0x47b5481dbefa4fa4,
];

#[rustfmt::skip]
const H512_224_INIT: [u64; 8] = [
    0x8c3d37c819544da2, 0x73e1996689dcd4d6,
    0x1dfab7ae32ff9c82, 0x679dd514582f9fcf,
    0x0f6d2b697bd44da8, 0x77e36f7304c48942,
    0x3f9d85a86a1d36c8, 0x1112e6ad91d692a1,
];

#[rustfmt::skip]
const H512_256_INIT: [u64; 8] = [
    0x22312194fc2bf72c, 0x9f555fa3c84c64c2,
    0x2393b86b6f53b151, 0x963877195940eabd,
    0x96283ee2a88effe3, 0xbe5e1e2553863992,
    0x2b0199fc2c85b8aa, 0x0eb72ddc81c52ca2,
];

fn sha512_pad(msg: &[u8]) -> Vec<u8> {
    let len_bits: u128 = (msg.len() as u128)
        .checked_mul(8)
        .expect("message too long for SHA-512");
    let mut padded = msg.to_vec();
    padded.push(0x80);
    while padded.len() % 128 != 112 {
        padded.push(0x00);
    }
    padded.extend_from_slice(&len_bits.to_be_bytes());
    padded
}

fn sha512_compress(state: &mut [u64; 8], block: &[u8; 128]) {
    let mut w = [0u64; 80];
    for i in 0..16 {
        w[i] = u64::from_be_bytes(block[8 * i..8 * i + 8].try_into().unwrap());
    }
    for i in 16..80 {
        let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
        let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for i in 0..80 {
        let sigma1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(sigma1)
            .wrapping_add(ch)
            .wrapping_add(K64[i])
            .wrapping_add(w[i]);
        let sigma0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = sigma0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

fn sha512_hash(msg: &[u8], mut state: [u64; 8]) -> [u64; 8] {
    let padded = sha512_pad(msg);
    for chunk in padded.chunks_exact(128) {
        sha512_compress(&mut state, chunk.try_into().unwrap());
    }
    state
}

macro_rules! sha512_variant {
    ($name:ident, $init:expr, $out_bytes:literal) => {
        pub fn $name(msg: &[u8]) -> [u8; $out_bytes] {
            let state = sha512_hash(msg, $init);
            let mut out = [0u8; $out_bytes];
            let mut cursor = 0usize;
            'outer: for word in &state {
                for byte in word.to_be_bytes() {
                    if cursor == $out_bytes {
                        break 'outer;
                    }
                    out[cursor] = byte;
                    cursor += 1;
                }
            }
            out
        }
    };
}

sha512_variant!(sha512, H512_INIT, 64);
sha512_variant!(sha384, H384_INIT, 48);
sha512_variant!(sha512_224, H512_224_INIT, 28);
sha512_variant!(sha512_256, H512_256_INIT, 32);

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::LinearSieve;

    macro_rules! hash_test {
        ($test_name:ident, $fn:ident, $out:literal, $input:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let digest = $fn($input);
                let expected: [u8; $out] = $expected;
                assert_eq!(digest, expected);
            }
        };
    }

    // ── Constant correctness ─────────────────────────────────────────────────
    // Verifies K32 and H256_INIT against their mathematical definitions:
    // cube-root / square-root fractional bits of the first N primes.

    #[test]
    fn test_constants() {
        let mut ls = LinearSieve::new();
        ls.prepare(311).unwrap();
        assert_eq!(64, ls.primes.len());
        assert_eq!(311, ls.primes[63]);

        let float_len = 52u32;
        let constant_len = 32u32;

        for (pos, &k) in K32.iter().enumerate() {
            let a: f64 = ls.primes[pos] as f64;
            let bits = a.cbrt().to_bits();
            let exp = (bits >> float_len) as u32;
            let k_ref = ((bits & ((1u64 << float_len) - 1))
                >> (float_len - constant_len + 1023 - exp)) as u32;
            assert_eq!(k, k_ref, "K32[{pos}] mismatch");
        }

        for (pos, &h) in H256_INIT.iter().enumerate() {
            let a: f64 = ls.primes[pos] as f64;
            let bits = a.sqrt().to_bits();
            let exp = (bits >> float_len) as u32;
            let h_ref = ((bits & ((1u64 << float_len) - 1))
                >> (float_len - constant_len + 1023 - exp)) as u32;
            assert_eq!(h, h_ref, "H256_INIT[{pos}] mismatch");
        }
    }

    // ── SHA-256 (FIPS 180-4) ─────────────────────────────────────────────────

    hash_test!(
        sha256_empty,
        sha256,
        32,
        b"",
        [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55
        ]
    );

    hash_test!(
        sha256_abc,
        sha256,
        32,
        b"abc",
        [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x00, 0x15, 0xad
        ]
    );

    hash_test!(
        sha256_448bit,
        sha256,
        32,
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        [
            0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e,
            0x60, 0x39, 0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 0xf6, 0xec, 0xed, 0xd4,
            0x19, 0xdb, 0x06, 0xc1
        ]
    );

    // ── SHA-224 (FIPS 180-4) ─────────────────────────────────────────────────

    hash_test!(
        sha224_empty,
        sha224,
        28,
        b"",
        [
            0xd1, 0x4a, 0x02, 0x8c, 0x2a, 0x3a, 0x2b, 0xc9, 0x47, 0x61, 0x02, 0xbb, 0x28, 0x82,
            0x34, 0xc4, 0x15, 0xa2, 0xb0, 0x1f, 0x82, 0x8e, 0xa6, 0x2a, 0xc5, 0xb3, 0xe4, 0x2f
        ]
    );

    hash_test!(
        sha224_abc,
        sha224,
        28,
        b"abc",
        [
            0x23, 0x09, 0x7d, 0x22, 0x34, 0x05, 0xd8, 0x22, 0x86, 0x42, 0xa4, 0x77, 0xbd, 0xa2,
            0x55, 0xb3, 0x2a, 0xad, 0xbc, 0xe4, 0xbd, 0xa0, 0xb3, 0xf7, 0xe3, 0x6c, 0x9d, 0xa7
        ]
    );

    hash_test!(
        sha224_448bit,
        sha224,
        28,
        b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
        [
            0x75, 0x38, 0x8b, 0x16, 0x51, 0x27, 0x76, 0xcc, 0x5d, 0xba, 0x5d, 0xa1, 0xfd, 0x89,
            0x01, 0x50, 0xb0, 0xc6, 0x45, 0x5c, 0xb4, 0xf5, 0x8b, 0x19, 0x52, 0x52, 0x25, 0x25
        ]
    );

    // ── SHA-512 (FIPS 180-4) ─────────────────────────────────────────────────

    hash_test!(
        sha512_empty,
        sha512,
        64,
        b"",
        [
            0xcf, 0x83, 0xe1, 0x35, 0x7e, 0xef, 0xb8, 0xbd, 0xf1, 0x54, 0x28, 0x50, 0xd6, 0x6d,
            0x80, 0x07, 0xd6, 0x20, 0xe4, 0x05, 0x0b, 0x57, 0x15, 0xdc, 0x83, 0xf4, 0xa9, 0x21,
            0xd3, 0x6c, 0xe9, 0xce, 0x47, 0xd0, 0xd1, 0x3c, 0x5d, 0x85, 0xf2, 0xb0, 0xff, 0x83,
            0x18, 0xd2, 0x87, 0x7e, 0xec, 0x2f, 0x63, 0xb9, 0x31, 0xbd, 0x47, 0x41, 0x7a, 0x81,
            0xa5, 0x38, 0x32, 0x7a, 0xf9, 0x27, 0xda, 0x3e
        ]
    );

    hash_test!(
        sha512_abc,
        sha512,
        64,
        b"abc",
        [
            0xdd, 0xaf, 0x35, 0xa1, 0x93, 0x61, 0x7a, 0xba, 0xcc, 0x41, 0x73, 0x49, 0xae, 0x20,
            0x41, 0x31, 0x12, 0xe6, 0xfa, 0x4e, 0x89, 0xa9, 0x7e, 0xa2, 0x0a, 0x9e, 0xee, 0xe6,
            0x4b, 0x55, 0xd3, 0x9a, 0x21, 0x92, 0x99, 0x2a, 0x27, 0x4f, 0xc1, 0xa8, 0x36, 0xba,
            0x3c, 0x23, 0xa3, 0xfe, 0xeb, 0xbd, 0x45, 0x4d, 0x44, 0x23, 0x64, 0x3c, 0xe8, 0x0e,
            0x2a, 0x9a, 0xc9, 0x4f, 0xa5, 0x4c, 0xa4, 0x9f
        ]
    );

    hash_test!(
        sha512_896bit, sha512, 64,
        b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
        [0x8e,0x95,0x9b,0x75,0xda,0xe3,0x13,0xda,0x8c,0xf4,0xf7,0x28,0x14,0xfc,0x14,0x3f,
         0x8f,0x77,0x79,0xc6,0xeb,0x9f,0x7f,0xa1,0x72,0x99,0xae,0xad,0xb6,0x88,0x90,0x18,
         0x50,0x1d,0x28,0x9e,0x49,0x00,0xf7,0xe4,0x33,0x1b,0x99,0xde,0xc4,0xb5,0x43,0x3a,
         0xc7,0xd3,0x29,0xee,0xb6,0xdd,0x26,0x54,0x5e,0x96,0xe5,0x5b,0x87,0x4b,0xe9,0x09]
    );

    // ── SHA-384 (FIPS 180-4) ─────────────────────────────────────────────────

    hash_test!(
        sha384_empty,
        sha384,
        48,
        b"",
        [
            0x38, 0xb0, 0x60, 0xa7, 0x51, 0xac, 0x96, 0x38, 0x4c, 0xd9, 0x32, 0x7e, 0xb1, 0xb1,
            0xe3, 0x6a, 0x21, 0xfd, 0xb7, 0x11, 0x14, 0xbe, 0x07, 0x43, 0x4c, 0x0c, 0xc7, 0xbf,
            0x63, 0xf6, 0xe1, 0xda, 0x27, 0x4e, 0xde, 0xbf, 0xe7, 0x6f, 0x65, 0xfb, 0xd5, 0x1a,
            0xd2, 0xf1, 0x48, 0x98, 0xb9, 0x5b
        ]
    );

    hash_test!(
        sha384_abc,
        sha384,
        48,
        b"abc",
        [
            0xcb, 0x00, 0x75, 0x3f, 0x45, 0xa3, 0x5e, 0x8b, 0xb5, 0xa0, 0x3d, 0x69, 0x9a, 0xc6,
            0x50, 0x07, 0x27, 0x2c, 0x32, 0xab, 0x0e, 0xde, 0xd1, 0x63, 0x1a, 0x8b, 0x60, 0x5a,
            0x43, 0xff, 0x5b, 0xed, 0x80, 0x86, 0x07, 0x2b, 0xa1, 0xe7, 0xcc, 0x23, 0x58, 0xba,
            0xec, 0xa1, 0x34, 0xc8, 0x25, 0xa7
        ]
    );

    hash_test!(
        sha384_896bit, sha384, 48,
        b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
        [0x09,0x33,0x0c,0x33,0xf7,0x11,0x47,0xe8,0x3d,0x19,0x2f,0xc7,0x82,0xcd,0x1b,0x47,
         0x53,0x11,0x1b,0x17,0x3b,0x3b,0x05,0xd2,0x2f,0xa0,0x80,0x86,0xe3,0xb0,0xf7,0x12,
         0xfc,0xc7,0xc7,0x1a,0x55,0x7e,0x2d,0xb9,0x66,0xc3,0xe9,0xfa,0x91,0x74,0x60,0x39]
    );

    // ── SHA-512/256 (FIPS 180-4) ─────────────────────────────────────────────

    hash_test!(
        sha512_256_empty,
        sha512_256,
        32,
        b"",
        [
            0xc6, 0x72, 0xb8, 0xd1, 0xef, 0x56, 0xed, 0x28, 0xab, 0x87, 0xc3, 0x62, 0x2c, 0x51,
            0x14, 0x06, 0x9b, 0xdd, 0x3a, 0xd7, 0xb8, 0xf9, 0x73, 0x74, 0x98, 0xd0, 0xc0, 0x1e,
            0xce, 0xf0, 0x96, 0x7a
        ]
    );

    hash_test!(
        sha512_256_abc,
        sha512_256,
        32,
        b"abc",
        [
            0x53, 0x04, 0x8e, 0x26, 0x81, 0x94, 0x1e, 0xf9, 0x9b, 0x2e, 0x29, 0xb7, 0x6b, 0x4c,
            0x7d, 0xab, 0xe4, 0xc2, 0xd0, 0xc6, 0x34, 0xfc, 0x6d, 0x46, 0xe0, 0xe2, 0xf1, 0x31,
            0x07, 0xe7, 0xaf, 0x23
        ]
    );

    // ── SHA-512/224 (FIPS 180-4) ─────────────────────────────────────────────

    hash_test!(
        sha512_224_empty,
        sha512_224,
        28,
        b"",
        [
            0x6e, 0xd0, 0xdd, 0x02, 0x80, 0x6f, 0xa8, 0x9e, 0x25, 0xde, 0x06, 0x0c, 0x19, 0xd3,
            0xac, 0x86, 0xca, 0xbb, 0x87, 0xd6, 0xa0, 0xdd, 0xd0, 0x5c, 0x33, 0x3b, 0x84, 0xf4
        ]
    );

    hash_test!(
        sha512_224_abc,
        sha512_224,
        28,
        b"abc",
        [
            0x46, 0x34, 0x27, 0x0f, 0x70, 0x7b, 0x6a, 0x54, 0xda, 0xae, 0x75, 0x30, 0x46, 0x08,
            0x42, 0xe2, 0x0e, 0x37, 0xed, 0x26, 0x5c, 0xee, 0xe9, 0xa4, 0x3e, 0x89, 0x24, 0xaa
        ]
    );
}
