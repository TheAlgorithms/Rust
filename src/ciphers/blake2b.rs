use std::cmp::{max, min};
use std::convert::{TryFrom, TryInto};

type Word = u64;

const BB: usize = 128;

const KK_MAX: usize = 64;
const NN_MAX: u8 = 64;

const U64BITS: usize = u64::BITS as usize;
const U64BYTES: usize = U64BITS / 8;

type Block = [Word; BB / U64BYTES];

const R1: u32 = 32;
const R2: u32 = 24;
const R3: u32 = 16;
const R4: u32 = 63;

const IV: [Word; 8] = [
    0x6A09E667F3BCC908,
    0xBB67AE8584CAA73B,
    0x3C6EF372FE94F82B,
    0xA54FF53A5F1D36F1,
    0x510E527FADE682D1,
    0x9B05688C2B3E6C1F,
    0x1F83D9ABFB41BD6B,
    0x5BE0CD19137E2179,
];

const SIGMA: [[usize; 16]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
];

#[inline]
const fn blank_block() -> Block {
    [0u64; BB / U64BYTES]
}

#[inline]
fn add(a: &mut Word, b: Word) {
    *a = a.overflowing_add(b).0;
}

#[inline]
const fn ceil(dividend: usize, divisor: usize) -> usize {
    (dividend / divisor) + ((dividend % divisor != 0) as usize)
}

fn g(v: &mut [Word; 16], a: usize, b: usize, c: usize, d: usize, x: Word, y: Word) {
    let rc = [R1, R2, R3, R4];

    let mixing = [x, y];

    for (m, r) in mixing.into_iter().zip(rc.chunks(2)) {
        let v_b = v[b];
        add(&mut v[a], v_b);
        add(&mut v[a], m);

        v[d] = (v[d] ^ v[a]).rotate_right(r[0]);

        let v_d = v[d];
        add(&mut v[c], v_d);

        v[b] = (v[b] ^ v[c]).rotate_right(r[1]);
    }
}

fn f(h: &mut [Word; 8], m: Block, t: u128, flag: bool) {
    let mut v: [Word; 16] = [0; 16];

    for (i, (h_i, iv_i)) in h.iter().zip(IV.iter()).enumerate() {
        v[i] = *h_i;
        v[i + 8] = *iv_i;
    }

    v[12] ^= (t % (u64::MAX as u128)) as u64;
    v[13] ^= (t >> 64) as u64;

    if flag {
        v[14] = !v[14];
    }

    for i in 0..12 {
        let s = SIGMA[i % 10];

        let mut s_index = 0;
        for j in 0..4 {
            g(
                &mut v,
                j,
                j + 4,
                j + 8,
                j + 12,
                m[s[s_index]],
                m[s[s_index + 1]],
            );

            s_index += 2;
        }

        let i1d = |col, row| {
            let col = col % 4;
            let row = row % 4;

            (row * 4) + col
        };

        for j in 0..4 {
            // Produces indeces for diagonals of a 4x4 matrix starting at 0,j
            let idx: Vec<usize> = (0..4).map(|n| i1d(j + n, n) as usize).collect();

            g(
                &mut v,
                idx[0],
                idx[1],
                idx[2],
                idx[3],
                m[s[s_index]],
                m[s[s_index + 1]],
            );

            s_index += 2;
        }
    }

    for (i, n) in h.iter_mut().enumerate() {
        *n ^= v[i] ^ v[i + 8];
    }
}

fn blake2(d: Vec<Block>, ll: u128, kk: Word, nn: Word) -> Vec<u8> {
    let mut h: [Word; 8] = IV
        .iter()
        .take(8)
        .copied()
        .collect::<Vec<Word>>()
        .try_into()
        .unwrap();

    h[0] ^= 0x01010000u64 ^ (kk << 8) ^ nn;

    if d.len() > 1 {
        for (i, w) in d.iter().enumerate().take(d.len() - 2) {
            f(&mut h, *w, (i as u128 + 1) * BB as u128, false);
        }
    }

    if kk == 0 {
        f(&mut h, d[d.len() - 1], ll, true);
    } else {
        f(&mut h, d[d.len() - 1], ll + BB as u128, true);
    }

    h.iter()
        .flat_map(|n| n.to_le_bytes())
        .take(nn as usize)
        .collect()
}

fn bytes_to_word(bytes: &[u8]) -> Word {
    if let Ok(arr) = <[u8; U64BYTES]>::try_from(bytes) {
        Word::from_le_bytes(arr)
    } else {
        let mut arr = [0u8; 8];
        for (a_i, b_i) in arr.iter_mut().zip(bytes) {
            *a_i = *b_i;
        }

        Word::from_le_bytes(arr)
    }
}

pub fn blake2b(m: &[u8], k: &[u8], nn: u8) -> Vec<u8> {
    let kk = min(k.len(), KK_MAX);
    let nn = min(nn, NN_MAX);

    let dd = max(ceil(kk, BB) + ceil(m.len(), BB), 1);

    let mut blocks: Vec<Block> = vec![blank_block(); dd];

    // Copy first 8 bytes of key into blocks
    for (w, c) in blocks[0].iter_mut().zip(k.chunks(U64BYTES)) {
        *w = bytes_to_word(c);
    }

    let first_index = (kk > 0) as usize;

    // Copy bytes from message into blocks
    for (i, c) in m.chunks(U64BYTES).enumerate() {
        let block_index = first_index + (i / BB);
        let word_in_block = (i % BB) / U64BYTES;

        blocks[block_index][word_in_block] = bytes_to_word(c);
    }

    blake2(blocks, m.len() as u128, kk as u64, nn as Word)
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! digest_test {
        ($fname:ident, $message:expr, $key:expr, $nn:literal, $expected:expr) => {
            #[test]
            fn $fname() {
                let digest = blake2b($message, $key, $nn);

                let expected = Vec::from($expected);

                assert_eq!(digest, expected);
            }
        };
    }

    digest_test!(
        blake2b_from_rfc,
        &[0x61, 0x62, 0x63],
        &[0; 0],
        64,
        [
            0xBA, 0x80, 0xA5, 0x3F, 0x98, 0x1C, 0x4D, 0x0D, 0x6A, 0x27, 0x97, 0xB6, 0x9F, 0x12,
            0xF6, 0xE9, 0x4C, 0x21, 0x2F, 0x14, 0x68, 0x5A, 0xC4, 0xB7, 0x4B, 0x12, 0xBB, 0x6F,
            0xDB, 0xFF, 0xA2, 0xD1, 0x7D, 0x87, 0xC5, 0x39, 0x2A, 0xAB, 0x79, 0x2D, 0xC2, 0x52,
            0xD5, 0xDE, 0x45, 0x33, 0xCC, 0x95, 0x18, 0xD3, 0x8A, 0xA8, 0xDB, 0xF1, 0x92, 0x5A,
            0xB9, 0x23, 0x86, 0xED, 0xD4, 0x00, 0x99, 0x23
        ]
    );
}
