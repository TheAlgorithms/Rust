const B: usize = 1600;

const W: usize = B / 25;
const L: usize = W.ilog2() as usize;

const U8BITS: usize = u8::BITS as usize;

// Macro for looping through the whole state array
macro_rules! iterate {
    ( $x:ident, $y:ident, $z:ident => $b:block ) => {
        for $y in 0..5 {
            for $x in 0..5 {
                for $z in 0..W {
                    $b
                }
            }
        }
    };
}

fn h2b(h: &[u8], n: usize) -> Vec<bool> {
    let mut bits = Vec::with_capacity(h.len() * U8BITS);

    for byte in h {
        for i in 0..u8::BITS {
            let mask: u8 = 1 << i;

            bits.push((byte & mask) != 0);
        }
    }

    assert!(bits.len() == h.len() * U8BITS);

    bits.truncate(n);
    bits
}

fn b2h(s: &[bool]) -> Vec<u8> {
    let m = if s.len() % U8BITS != 0 {
        (s.len() / 8) + 1
    } else {
        s.len() / 8
    };
    let mut bytes = vec![0u8; m];

    for (i, bit) in s.iter().enumerate() {
        let byte_index = i / U8BITS;
        let mask = (*bit as u8) << (i % U8BITS);

        bytes[byte_index] |= mask;
    }

    bytes
}

type PadFn = fn(isize, isize) -> Vec<bool>;
type SpongeFn = fn(&[bool]) -> [bool; B];

type State = [[[bool; W]; 5]; 5];

fn state_new() -> State {
    [[[false; W]; 5]; 5]
}

fn state_fill(dest: &mut State, bits: &[bool]) {
    let mut i = 0usize;

    iterate!(x, y, z => {
        if i >= bits.len() { return; }
        dest[x][y][z] = bits[i];
        i += 1;
    });
}

fn state_copy(dest: &mut State, src: &State) {
    iterate!(x, y, z => {
        dest[x][y][z] = src[x][y][z];
    });
}

fn state_dump(state: &State) -> [bool; B] {
    let mut bits = [false; B];

    let mut i = 0usize;

    iterate!(x, y, z => {
        bits[i] = state[x][y][z];
        i += 1;
    });

    bits
}

/// XORs the state with the parities of two columns in the state array
fn theta(state: &mut State) {
    let mut c = [[false; W]; 5];
    let mut d = [[false; W]; 5];

    // Assign values of C[x,z]
    for x in 0..5 {
        for z in 0..W {
            c[x][z] = state[x][0][z];

            for y in 1..5 {
                c[x][z] ^= state[x][y][z];
            }
        }
    }

    // Assign values of D[x,z]
    for x in 0..5 {
        for z in 0..W {
            let x1 = (x as isize - 1).rem_euclid(5) as usize;
            let z2 = (z as isize - 1).rem_euclid(W as isize) as usize;

            d[x][z] = c[x1][z] ^ c[(x + 1) % 5][z2];
        }
    }

    // Xor values of D[x,z] into our state array
    iterate!(x, y, z => {
        state[x][y][z] ^= d[x][z];
    });
}

/// Rotates each lane by an offset depending of the x and y indeces
fn rho(state: &mut State) {
    let mut new_state = state_new();

    for z in 0..W {
        new_state[0][0][z] = state[0][0][z];
    }

    let mut x = 1;
    let mut y = 0;

    for t in 0..=23isize {
        for z in 0..W {
            let z_offset: isize = ((t + 1) * (t + 2)) / 2;
            let new_z = (z as isize - z_offset).rem_euclid(W as isize) as usize;

            new_state[x][y][z] = state[x][y][new_z];
        }

        let old_y = y;
        y = ((2 * x) + (3 * y)) % 5;
        x = old_y;
    }

    state_copy(state, &new_state);
}

/// Rearrange the positions of the lanes of the state array
fn pi(state: &mut State) {
    let mut new_state = state_new();

    iterate!(x, y, z => {
        new_state[x][y][z] = state[(x + (3 * y)) % 5][x][z];
    });

    state_copy(state, &new_state);
}

fn chi(state: &mut State) {
    let mut new_state = state_new();

    iterate!(x, y, z => {
        new_state[x][y][z] = state[x][y][z] ^ ((state[(x + 1) % 5][y][z] ^ true) & state[(x + 2) % 5][y][z]);
    });

    state_copy(state, &new_state);
}

/// Calculates the round constant depending on what the round number is
fn rc(t: u8) -> bool {
    let mut b1: u16;
    let mut b2: u16;
    let mut r: u16 = 0x80; // tread r as an array of bits

    //if t % 0xFF == 0 { return true; }

    for _i in 0..(t % 255) {
        b1 = r >> 8;
        b2 = r & 1;
        r |= (b1 ^ b2) << 8;

        b1 = (r >> 4) & 1;
        r &= 0x1EF; // clear r[4]
        r |= (b1 ^ b2) << 4;

        b1 = (r >> 3) & 1;
        r &= 0x1F7; // clear r[3]
        r |= (b1 ^ b2) << 3;

        b1 = (r >> 2) & 1;
        r &= 0x1FB; // clear r[2]
        r |= (b1 ^ b2) << 2;

        r >>= 1;
    }

    (r >> 7) != 0
}

/// Applies the round constant to the first lane of the state array
fn iota(state: &mut State, i_r: u8) {
    let mut rc_arr = [false; W];

    for j in 0..=L {
        rc_arr[(1 << j) - 1] = rc((j as u8) + (7 * i_r));
    }

    for (z, bit) in rc_arr.iter().enumerate() {
        state[0][0][z] ^= *bit;
    }
}

fn rnd(state: &mut State, i_r: u8) {
    theta(state);
    rho(state);
    pi(state);
    chi(state);
    iota(state, i_r);
}

fn keccak_f(bits: &[bool]) -> [bool; B] {
    let n_r = 12 + (2 * L);

    let mut state = state_new();
    state_fill(&mut state, bits);

    for i_r in 0..n_r {
        rnd(&mut state, i_r as u8);
    }

    state_dump(&state)
}

fn pad101(x: isize, m: isize) -> Vec<bool> {
    let mut j = -m - 2;

    while j < 0 {
        j += x;
    }

    j %= x;

    let mut ret = vec![false; (j as usize) + 2];
    *ret.first_mut().unwrap() = true;
    *ret.last_mut().unwrap() = true;

    ret
}

fn sponge(f: SpongeFn, pad: PadFn, r: usize, n: &[bool], d: usize) -> Vec<bool> {
    let mut p = Vec::from(n);
    p.append(&mut pad(r as isize, n.len() as isize));

    assert!(r < B);

    let mut s = [false; B];
    for chunk in p.chunks(r) {
        for (s_i, c_i) in s.iter_mut().zip(chunk) {
            *s_i ^= c_i;
        }

        s = f(&s);
    }

    let mut z = Vec::<bool>::new();
    while z.len() < d {
        z.extend(&s);

        s = f(&s);
    }

    z.truncate(d);
    z
}

fn keccak(c: usize, n: &[bool], d: usize) -> Vec<bool> {
    sponge(keccak_f, pad101, B - c, n, d)
}

/// Macro to implement all sha3 hash functions as they only differ in digest size
macro_rules! sha3 {
    ($name:ident, $n:literal) => {
        pub fn $name(m: &[u8]) -> [u8; ($n / U8BITS)] {
            let mut temp = h2b(m, m.len() * U8BITS);
            temp.append(&mut vec![false, true]);

            temp = keccak($n * 2, &temp, $n);

            let mut ret = [0u8; ($n / U8BITS)];

            let temp = b2h(&temp);
            assert!(temp.len() == $n / U8BITS);

            for (i, byte) in temp.iter().enumerate() {
                ret[i] = *byte;
            }

            ret
        }
    };
}

sha3!(sha3_224, 224);
sha3!(sha3_256, 256);
sha3!(sha3_384, 384);
sha3!(sha3_512, 512);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! digest_test {
        ($fname:ident, $hash:ident, $size:literal, $message:expr, $expected:expr) => {
            #[test]
            fn $fname() {
                let digest = $hash(&$message);

                let expected: [u8; $size / U8BITS] = $expected;

                assert_eq!(digest, expected);
            }
        };
    }

    digest_test!(
        sha3_224_0,
        sha3_224,
        224,
        [0; 0],
        [
            0x6b, 0x4e, 0x03, 0x42, 0x36, 0x67, 0xdb, 0xb7, 0x3b, 0x6e, 0x15, 0x45, 0x4f, 0x0e,
            0xb1, 0xab, 0xd4, 0x59, 0x7f, 0x9a, 0x1b, 0x07, 0x8e, 0x3f, 0x5b, 0x5a, 0x6b, 0xc7,
        ]
    );

    digest_test!(
        sha3_224_8,
        sha3_224,
        224,
        [1u8],
        [
            0x48, 0x82, 0x86, 0xd9, 0xd3, 0x27, 0x16, 0xe5, 0x88, 0x1e, 0xa1, 0xee, 0x51, 0xf3,
            0x6d, 0x36, 0x60, 0xd7, 0x0f, 0x0d, 0xb0, 0x3b, 0x3f, 0x61, 0x2c, 0xe9, 0xed, 0xa4,
        ]
    );

    digest_test!(
        sha3_256_0,
        sha3_256,
        256,
        [0; 0],
        [
            0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61,
            0xd6, 0x62, 0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8, 0x0a, 0x4b,
            0x80, 0xf8, 0x43, 0x4a,
        ]
    );

    digest_test!(
        sha3_256_8,
        sha3_256,
        256,
        [0xe9u8],
        [
            0xf0, 0xd0, 0x4d, 0xd1, 0xe6, 0xcf, 0xc2, 0x9a, 0x44, 0x60, 0xd5, 0x21, 0x79, 0x68,
            0x52, 0xf2, 0x5d, 0x9e, 0xf8, 0xd2, 0x8b, 0x44, 0xee, 0x91, 0xff, 0x5b, 0x75, 0x9d,
            0x72, 0xc1, 0xe6, 0xd6,
        ]
    );

    digest_test!(
        sha3_384_0,
        sha3_384,
        384,
        [0; 0],
        [
            0x0c, 0x63, 0xa7, 0x5b, 0x84, 0x5e, 0x4f, 0x7d, 0x01, 0x10, 0x7d, 0x85, 0x2e, 0x4c,
            0x24, 0x85, 0xc5, 0x1a, 0x50, 0xaa, 0xaa, 0x94, 0xfc, 0x61, 0x99, 0x5e, 0x71, 0xbb,
            0xee, 0x98, 0x3a, 0x2a, 0xc3, 0x71, 0x38, 0x31, 0x26, 0x4a, 0xdb, 0x47, 0xfb, 0x6b,
            0xd1, 0xe0, 0x58, 0xd5, 0xf0, 0x04,
        ]
    );

    digest_test!(
        sha3_384_8,
        sha3_384,
        384,
        [0x80u8],
        [
            0x75, 0x41, 0x38, 0x48, 0x52, 0xe1, 0x0f, 0xf1, 0x0d, 0x5f, 0xb6, 0xa7, 0x21, 0x3a,
            0x4a, 0x6c, 0x15, 0xcc, 0xc8, 0x6d, 0x8b, 0xc1, 0x06, 0x8a, 0xc0, 0x4f, 0x69, 0x27,
            0x71, 0x42, 0x94, 0x4f, 0x4e, 0xe5, 0x0d, 0x91, 0xfd, 0xc5, 0x65, 0x53, 0xdb, 0x06,
            0xb2, 0xf5, 0x03, 0x9c, 0x8a, 0xb7,
        ]
    );

    digest_test!(
        sha3_512_0,
        sha3_512,
        512,
        [0u8; 0],
        [
            0xa6, 0x9f, 0x73, 0xcc, 0xa2, 0x3a, 0x9a, 0xc5, 0xc8, 0xb5, 0x67, 0xdc, 0x18, 0x5a,
            0x75, 0x6e, 0x97, 0xc9, 0x82, 0x16, 0x4f, 0xe2, 0x58, 0x59, 0xe0, 0xd1, 0xdc, 0xc1,
            0x47, 0x5c, 0x80, 0xa6, 0x15, 0xb2, 0x12, 0x3a, 0xf1, 0xf5, 0xf9, 0x4c, 0x11, 0xe3,
            0xe9, 0x40, 0x2c, 0x3a, 0xc5, 0x58, 0xf5, 0x00, 0x19, 0x9d, 0x95, 0xb6, 0xd3, 0xe3,
            0x01, 0x75, 0x85, 0x86, 0x28, 0x1d, 0xcd, 0x26,
        ]
    );

    digest_test!(
        sha3_512_8,
        sha3_512,
        512,
        [0xe5u8],
        [
            0x15, 0x02, 0x40, 0xba, 0xf9, 0x5f, 0xb3, 0x6f, 0x8c, 0xcb, 0x87, 0xa1, 0x9a, 0x41,
            0x76, 0x7e, 0x7a, 0xed, 0x95, 0x12, 0x50, 0x75, 0xa2, 0xb2, 0xdb, 0xba, 0x6e, 0x56,
            0x5e, 0x1c, 0xe8, 0x57, 0x5f, 0x2b, 0x04, 0x2b, 0x62, 0xe2, 0x9a, 0x04, 0xe9, 0x44,
            0x03, 0x14, 0xa8, 0x21, 0xc6, 0x22, 0x41, 0x82, 0x96, 0x4d, 0x8b, 0x55, 0x7b, 0x16,
            0xa4, 0x92, 0xb3, 0x80, 0x6f, 0x4c, 0x39, 0xc1
        ]
    );
}
