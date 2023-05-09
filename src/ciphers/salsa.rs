macro_rules! quarter_round {
    ($v1:expr,$v2:expr,$v3:expr,$v4:expr) => {
        $v2 ^= ($v1.wrapping_add($v4).rotate_left(7));
        $v3 ^= ($v2.wrapping_add($v1).rotate_left(9));
        $v4 ^= ($v3.wrapping_add($v2).rotate_left(13));
        $v1 ^= ($v4.wrapping_add($v3).rotate_left(18));
    };
}

/// This is a `Salsa20` implementation based on <https://en.wikipedia.org/wiki/Salsa20>\
/// `Salsa20` is a stream cipher developed by Daniel J. Bernstein.\
/// To use it, the `salsa20` function should be called with appropriate parameters and the
/// output of the function should be XORed with plain text.
///
/// `salsa20` function takes as input an array of 16 32-bit integers (512 bits)
/// of which 128 bits is the constant 'expand 32-byte k', 256 bits is the key,
/// and 128 bits are nonce and counter. It is up to the user to determine how
/// many bits each of nonce and counter take, but a default of 64 bits each
/// seems to be a sane choice.
///
/// The 16 input numbers can be thought of as the elements of a 4x4 matrix like
/// the one bellow, on which we do the main operations of the cipher.
///
/// ```text
/// +----+----+----+----+
/// | 00 | 01 | 02 | 03 |
/// +----+----+----+----+
/// | 04 | 05 | 06 | 07 |
/// +----+----+----+----+
/// | 08 | 09 | 10 | 11 |
/// +----+----+----+----+
/// | 12 | 13 | 14 | 15 |
/// +----+----+----+----+
/// ```
///
/// As per the diagram bellow, `input[0, 5, 10, 15]` are the constants mentioned
/// above, `input[1, 2, 3, 4, 11, 12, 13, 14]` is filled with the key, and
/// `input[6, 7, 8, 9]` should be filled with nonce and counter values. The output
/// of the function is stored in `output` variable and can be XORed with the
/// plain text to produce the cipher text.
///
/// ```text
/// +------+------+------+------+
/// |      |      |      |      |
/// | C[0] | key1 | key2 | key3 |
/// |      |      |      |      |
/// +------+------+------+------+
/// |      |      |      |      |
/// | key4 | C[1] | no1  | no2  |
/// |      |      |      |      |
/// +------+------+------+------+
/// |      |      |      |      |
/// | ctr1 | ctr2 | C[2] | key5 |
/// |      |      |      |      |
/// +------+------+------+------+
/// |      |      |      |      |
/// | key6 | key7 | key8 | C[3] |
/// |      |      |      |      |
/// +------+------+------+------+
/// ```
pub fn salsa20(input: &[u32; 16], output: &mut [u32; 16]) {
    output.copy_from_slice(&input[..]);
    for _ in 0..10 {
        // Odd round
        quarter_round!(output[0], output[4], output[8], output[12]); // column 1
        quarter_round!(output[5], output[9], output[13], output[1]); // column 2
        quarter_round!(output[10], output[14], output[2], output[6]); // column 3
        quarter_round!(output[15], output[3], output[7], output[11]); // column 4

        // Even round
        quarter_round!(output[0], output[1], output[2], output[3]); // row 1
        quarter_round!(output[5], output[6], output[7], output[4]); // row 2
        quarter_round!(output[10], output[11], output[8], output[9]); // row 3
        quarter_round!(output[15], output[12], output[13], output[14]); // row 4
    }
    for (a, &b) in output.iter_mut().zip(input.iter()) {
        *a = a.wrapping_add(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    const C: [u32; 4] = [0x65787061, 0x6e642033, 0x322d6279, 0x7465206b];

    fn output_hex(inp: &[u32; 16]) -> String {
        let mut res = String::new();
        res.reserve(512 / 4);
        for &x in inp {
            write!(&mut res, "{x:08x}").unwrap();
        }
        res
    }
    #[test]
    // test vector 1
    fn basic_tv1() {
        let mut inp = [0u32; 16];
        let mut out = [0u32; 16];
        inp[0] = C[0];
        inp[1] = 0x01020304; // 1, 2, 3, 4
        inp[2] = 0x05060708; // 5, 6, 7, 8, ...
        inp[3] = 0x090a0b0c;
        inp[4] = 0x0d0e0f10;
        inp[5] = C[1];
        inp[6] = 0x65666768; // 101, 102, 103, 104
        inp[7] = 0x696a6b6c; // 105, 106, 107, 108, ...
        inp[8] = 0x6d6e6f70;
        inp[9] = 0x71727374;
        inp[10] = C[2];
        inp[11] = 0xc9cacbcc; // 201, 202, 203, 204
        inp[12] = 0xcdcecfd0; // 205, 206, 207, 208, ...
        inp[13] = 0xd1d2d3d4;
        inp[14] = 0xd5d6d7d8;
        inp[15] = C[3];
        salsa20(&inp, &mut out);
        // Checked with wikipedia implementation, does not agree with
        // "https://cr.yp.to/snuffle/spec.pdf"
        assert_eq!(
            output_hex(&out),
            concat!(
                "de1d6f8d91dbf69d0db4b70c8b4320d236694432896d98b05aa7b76d5738ca13",
                "04e5a170c8e479af1542ed2f30f26ba57da20203cfe955c66f4cc7a06dd34359"
            )
        );
    }
}
