/*!
 * SHA-2 256 bit implementation
 * This implementation is based on RFC6234
 * Keep in mind that the amount of data (in bits) processed should always be an
 * integer multiple of 8
 */

// The constants are tested to make sure they are correct
pub const H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// The following functions are implemented according to page 10 of RFC6234
#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ ((!x) & z)
}

#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline]
fn bsig0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline]
fn bsig1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline]
fn ssig0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline]
fn ssig1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

pub struct SHA256 {
    /// The current block to be processed, 512 bits long
    buffer: [u32; 16],
    /// Length (bits) of the message, should always be a multiple of 8
    length: u64,
    /// The current hash value. Note: this value is invalid unless `finalize`
    /// is called
    pub h: [u32; 8],
    /// Message schedule
    w: [u32; 64],
    pub finalized: bool,
    // Temporary values:
    round: [u32; 8],
}

fn process_block(h: &mut [u32; 8], w: &mut [u32; 64], round: &mut [u32; 8], buf: &[u32; 16]) {
    // Prepare the message schedule:
    w[..buf.len()].copy_from_slice(&buf[..]);
    for i in buf.len()..w.len() {
        w[i] = ssig1(w[i - 2])
            .wrapping_add(w[i - 7])
            .wrapping_add(ssig0(w[i - 15]))
            .wrapping_add(w[i - 16]);
    }
    round.copy_from_slice(h);
    for i in 0..w.len() {
        let t1 = round[7]
            .wrapping_add(bsig1(round[4]))
            .wrapping_add(ch(round[4], round[5], round[6]))
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let t2 = bsig0(round[0]).wrapping_add(maj(round[0], round[1], round[2]));
        round[7] = round[6];
        round[6] = round[5];
        round[5] = round[4];
        round[4] = round[3].wrapping_add(t1);
        round[3] = round[2];
        round[2] = round[1];
        round[1] = round[0];
        round[0] = t1.wrapping_add(t2);
    }
    for i in 0..h.len() {
        h[i] = h[i].wrapping_add(round[i]);
    }
}

impl SHA256 {
    pub fn new_default() -> Self {
        SHA256 {
            buffer: [0u32; 16],
            length: 0,
            h: H0,
            w: [0u32; 64],
            round: [0u32; 8],
            finalized: false,
        }
    }
    /// Note: buffer should be empty before calling this!
    pub fn process_block(&mut self, buf: &[u32; 16]) {
        process_block(&mut self.h, &mut self.w, &mut self.round, buf);
        self.length += 512;
    }

    pub fn update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        let offset = (((32 - (self.length & 31)) & 31) >> 3) as usize;
        let mut buf_ind = ((self.length & 511) >> 5) as usize;
        for (i, &byte) in data.iter().enumerate().take(offset) {
            self.buffer[buf_ind] ^= (byte as u32) << ((offset - i - 1) << 3);
        }
        self.length += (data.len() as u64) << 3;
        if offset > data.len() {
            return;
        }
        if offset > 0 {
            buf_ind += 1;
        }
        if data.len() > 3 {
            for i in (offset..(data.len() - 3)).step_by(4) {
                if buf_ind & 16 == 16 {
                    process_block(&mut self.h, &mut self.w, &mut self.round, &self.buffer);
                    buf_ind = 0;
                }
                self.buffer[buf_ind] = ((data[i] as u32) << 24)
                    ^ ((data[i + 1] as u32) << 16)
                    ^ ((data[i + 2] as u32) << 8)
                    ^ data[i + 3] as u32;
                buf_ind += 1;
            }
        }
        if buf_ind & 16 == 16 {
            process_block(&mut self.h, &mut self.w, &mut self.round, &self.buffer);
            buf_ind = 0;
        }
        self.buffer[buf_ind] = 0;
        let rem_ind = offset + ((data.len() - offset) & !0b11);
        for (i, &byte) in data[rem_ind..].iter().enumerate() {
            self.buffer[buf_ind] ^= (byte as u32) << ((3 - i) << 3);
        }
    }

    pub fn get_hash(&mut self) -> [u8; 32] {
        // we should first add a `1` bit to the end of the buffer, then we will
        // add enough 0s so that the length becomes (512k + 448). After that we
        // will append the binary representation of length to the data
        if !self.finalized {
            self.finalized = true;
            let clen = (self.length + 8) & 511;
            let num_0 = match clen.cmp(&448) {
                std::cmp::Ordering::Greater => (448 + 512 - clen) >> 3,
                _ => (448 - clen) >> 3,
            };
            let mut padding: Vec<u8> = vec![0_u8; (num_0 + 9) as usize];
            let len = padding.len();
            padding[0] = 0x80;
            padding[len - 8] = (self.length >> 56) as u8;
            padding[len - 7] = (self.length >> 48) as u8;
            padding[len - 6] = (self.length >> 40) as u8;
            padding[len - 5] = (self.length >> 32) as u8;
            padding[len - 4] = (self.length >> 24) as u8;
            padding[len - 3] = (self.length >> 16) as u8;
            padding[len - 2] = (self.length >> 8) as u8;
            padding[len - 1] = self.length as u8;
            self.update(&padding);
        }
        assert_eq!(self.length & 511, 0);
        let mut result = [0u8; 32];
        for i in (0..32).step_by(4) {
            result[i] = (self.h[i >> 2] >> 24) as u8;
            result[i + 1] = (self.h[i >> 2] >> 16) as u8;
            result[i + 2] = (self.h[i >> 2] >> 8) as u8;
            result[i + 3] = self.h[i >> 2] as u8;
        }
        result
    }
}

impl super::Hasher<32> for SHA256 {
    fn new_default() -> Self {
        SHA256::new_default()
    }

    fn update(&mut self, data: &[u8]) {
        self.update(data);
    }

    fn get_hash(&mut self) -> [u8; 32] {
        self.get_hash()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::math::LinearSieve;
    use std::fmt::Write;

    // Let's keep this utility function
    pub fn get_hash_string(hash: &[u8; 32]) -> String {
        let mut result = String::new();
        result.reserve(64);
        for &ch in hash {
            write!(&mut result, "{ch:02x}").unwrap();
        }
        result
    }

    #[test]
    fn test_constants() {
        let mut ls = LinearSieve::new();
        ls.prepare(311).unwrap();
        assert_eq!(64, ls.primes.len());
        assert_eq!(311, ls.primes[63]);

        let float_len = 52;
        let constant_len = 32;
        for (pos, &k) in K.iter().enumerate() {
            let a: f64 = ls.primes[pos] as f64;
            let bits = a.cbrt().to_bits();
            let exp = bits >> float_len; // The sign bit is already 0
                                         //(exp - 1023) can be bigger than 0, we must include more bits.
            let k_ref = ((bits & ((1_u64 << float_len) - 1))
                >> (float_len - constant_len + 1023 - exp)) as u32;
            assert_eq!(k, k_ref);
        }

        for (pos, &h) in H0.iter().enumerate() {
            let a: f64 = ls.primes[pos] as f64;
            let bits = a.sqrt().to_bits();
            let exp = bits >> float_len;
            let h_ref = ((bits & ((1_u64 << float_len) - 1))
                >> (float_len - constant_len + 1023 - exp)) as u32;
            assert_eq!(h, h_ref);
        }
    }

    // To test the hashes, you can use the following command on linux:
    // echo -n 'STRING' | sha256sum
    // the `-n` is because by default, echo adds a `\n` to its output

    #[test]
    fn empty() {
        let mut res = SHA256::new_default();
        assert_eq!(
            res.get_hash(),
            [
                0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
                0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
                0x78, 0x52, 0xb8, 0x55
            ]
        );
    }

    #[test]
    fn ascii() {
        let mut res = SHA256::new_default();
        res.update(b"The quick brown fox jumps over the lazy dog");
        assert_eq!(
            res.get_hash(),
            [
                0xD7, 0xA8, 0xFB, 0xB3, 0x07, 0xD7, 0x80, 0x94, 0x69, 0xCA, 0x9A, 0xBC, 0xB0, 0x08,
                0x2E, 0x4F, 0x8D, 0x56, 0x51, 0xE4, 0x6D, 0x3C, 0xDB, 0x76, 0x2D, 0x02, 0xD0, 0xBF,
                0x37, 0xC9, 0xE5, 0x92
            ]
        )
    }

    #[test]
    fn ascii_avalanche() {
        let mut res = SHA256::new_default();
        res.update(b"The quick brown fox jumps over the lazy dog.");
        assert_eq!(
            res.get_hash(),
            [
                0xEF, 0x53, 0x7F, 0x25, 0xC8, 0x95, 0xBF, 0xA7, 0x82, 0x52, 0x65, 0x29, 0xA9, 0xB6,
                0x3D, 0x97, 0xAA, 0x63, 0x15, 0x64, 0xD5, 0xD7, 0x89, 0xC2, 0xB7, 0x65, 0x44, 0x8C,
                0x86, 0x35, 0xFB, 0x6C
            ]
        );
        // Test if finalization is not repeated twice
        assert_eq!(
            res.get_hash(),
            [
                0xEF, 0x53, 0x7F, 0x25, 0xC8, 0x95, 0xBF, 0xA7, 0x82, 0x52, 0x65, 0x29, 0xA9, 0xB6,
                0x3D, 0x97, 0xAA, 0x63, 0x15, 0x64, 0xD5, 0xD7, 0x89, 0xC2, 0xB7, 0x65, 0x44, 0x8C,
                0x86, 0x35, 0xFB, 0x6C
            ]
        )
    }
    #[test]
    fn long_ascii() {
        let mut res = SHA256::new_default();
        let val = b"The quick brown fox jumps over the lazy dog.";
        for _ in 0..1000 {
            res.update(val);
        }
        let hash = res.get_hash();
        assert_eq!(
            &get_hash_string(&hash),
            "c264fca077807d391df72fadf39dd63be21f1823f65ca530c9637760eabfc18c"
        );
        let mut res = SHA256::new_default();
        let val = b"a";
        for _ in 0..999 {
            res.update(val);
        }
        let hash = res.get_hash();
        assert_eq!(
            &get_hash_string(&hash),
            "d9fe27f3d807a7c46467325f7189495e82b099ce2e14c5b16cc76697fa909f81"
        )
    }
    #[test]
    fn short_ascii() {
        let mut res = SHA256::new_default();
        let val = b"a";
        res.update(val);
        let hash = res.get_hash();
        assert_eq!(
            &get_hash_string(&hash),
            "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"
        );
    }
}
