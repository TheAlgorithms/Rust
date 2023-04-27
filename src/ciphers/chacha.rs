macro_rules! quarter_round {
    ($a:expr,$b:expr,$c:expr,$d:expr) => {
        $a = $a.wrapping_add($b);
        $d = ($d ^ $a).rotate_left(16);
        $c = $c.wrapping_add($d);
        $b = ($b ^ $c).rotate_left(12);
        $a = $a.wrapping_add($b);
        $d = ($d ^ $a).rotate_left(8);
        $c = $c.wrapping_add($d);
        $b = ($b ^ $c).rotate_left(7);
    };
}

#[allow(dead_code)]
// "expand 32-byte k", written in little-endian order
pub const C: [u32; 4] = [0x61707865, 0x3320646e, 0x79622d32, 0x6b206574];

/// ChaCha20 implementation based on RFC8439
///
/// ChaCha20 is a stream cipher developed independently by Daniel J. Bernstein.\
/// To use it, the `chacha20` function should be called with appropriate
/// parameters and the output of the function should be XORed with plain text.
///
/// `chacha20` function takes as input an array of 16 32-bit integers (512 bits)
/// of which 128 bits is the constant 'expand 32-byte k', 256 bits is the key,
/// and 128 bits are nonce and counter. According to RFC8439, the nonce should
/// be 96 bits long, which leaves 32 bits for the counter. Given that the block
/// length is 512 bits, this leaves enough counter values to encrypt 256GB of
/// data.
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
/// As per the diagram bellow, `input[0, 1, 2, 3]` are the constants mentioned
/// above, `input[4..=11]` is filled with the key, and `input[6..=9]` should be
/// filled with nonce and counter values. The output of the function is stored
/// in `output` variable and can be XORed with the plain text to produce the
/// cipher text.
///
/// ```text
/// +------+------+------+------+
/// |      |      |      |      |
/// | C[0] | C[1] | C[2] | C[3] |
/// |      |      |      |      |
/// +------+------+------+------+
/// |      |      |      |      |
/// | key0 | key1 | key2 | key3 |
/// |      |      |      |      |
/// +------+------+------+------+
/// |      |      |      |      |
/// | key4 | key5 | key6 | key7 |
/// |      |      |      |      |
/// +------+------+------+------+
/// |      |      |      |      |
/// | ctr0 | no.0 | no.1 | no.2 |
/// |      |      |      |      |
/// +------+------+------+------+
/// ```
///
/// Note that the constants, the key, and the nonce should be written in
/// little-endian order, meaning that for example if the key is 01:02:03:04
/// (in hex), it corresponds to the integer `0x04030201`. It is important to
/// know that the hex value of the counter is meaningless, and only its integer
/// value matters, and it should start with (for example) `0x00000000`, and then
/// `0x00000001` and so on until `0xffffffff`. Keep in mind that as soon as we get
/// from bytes to words, we stop caring about their representation in memory,
/// and we only need the math to be correct.
///
/// The output of the function can be used without any change, as long as the
/// plain text has the same endianness. For example if the plain text is
/// "hello world", and the first word of the output is `0x01020304`, then the
/// first byte of plain text ('h') should be XORed with the least-significant
/// byte of `0x01020304`, which is `0x04`.
pub fn chacha20(input: &[u32; 16], output: &mut [u32; 16]) {
    output.copy_from_slice(&input[..]);
    for _ in 0..10 {
        // Odd round (column round)
        quarter_round!(output[0], output[4], output[8], output[12]); //  column 1
        quarter_round!(output[1], output[5], output[9], output[13]); //  column 2
        quarter_round!(output[2], output[6], output[10], output[14]); // column 3
        quarter_round!(output[3], output[7], output[11], output[15]); // column 4

        // Even round (diagonal round)
        quarter_round!(output[0], output[5], output[10], output[15]); // diag 1
        quarter_round!(output[1], output[6], output[11], output[12]); // diag 2
        quarter_round!(output[2], output[7], output[8], output[13]); //  diag 3
        quarter_round!(output[3], output[4], output[9], output[14]); //  diag 4
    }
    for (a, &b) in output.iter_mut().zip(input.iter()) {
        *a = a.wrapping_add(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

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
        inp[1] = C[1];
        inp[2] = C[2];
        inp[3] = C[3];
        inp[4] = 0x03020100; // The key is 00:01:02:..:1f (hex)
        inp[5] = 0x07060504;
        inp[6] = 0x0b0a0908;
        inp[7] = 0x0f0e0d0c;
        inp[8] = 0x13121110;
        inp[9] = 0x17161514;
        inp[10] = 0x1b1a1918;
        inp[11] = 0x1f1e1d1c;
        inp[12] = 0x00000001; // The value of counter is 1 (an integer). Nonce:
        inp[13] = 0x09000000; // 00:00:00:09
        inp[14] = 0x4a000000; // 00:00:00:4a
        inp[15] = 0x00000000; // 00:00:00:00
        chacha20(&inp, &mut out);
        assert_eq!(
            output_hex(&out),
            concat!(
                "e4e7f11015593bd11fdd0f50c47120a3c7f4d1c70368c0339aaa22044e6cd4c3",
                "466482d209aa9f0705d7c214a2028bd9d19c12b5b94e16dee883d0cb4e3c50a2"
            )
        );
    }
}
