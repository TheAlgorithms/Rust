//! XTEA (Extended Tiny Encryption Algorithm) is a block cipher based on the Tiny Encryption algorihtm.
//! It was specifically made to improve on its weaknesses.

// This is so the formulas in the en-/decryption parts are more clear.
#![allow(unused_parens)]

pub fn xtea_encrypt(plaintext: &mut [u8], key: &[u32; 4]) {
    for i in 0..(plaintext.len() / 8) {
        let mut block: [u8; 8] = [0; 8];
        for (index, byte) in plaintext[(i * 8)..((i + 1) * 8)].iter().enumerate() {
            block[index] = *byte;
        }
        block = xtea_encrypt_bytes(&block, key);
        for (index, byte) in block.iter().enumerate() {
            plaintext[index + (i * 8)] = *byte;
        }
    }
}

pub fn xtea_decrypt(cyphertext: &mut [u8], key: &[u32; 4]) {
    for i in 0..(cyphertext.len() / 8) {
        let mut block: [u8; 8] = [0; 8];
        for (index, byte) in cyphertext[(i * 8)..((i + 1) * 8)].iter().enumerate() {
            block[index] = *byte;
        }
        block = xtea_decrypt_bytes(&block, key);
        for (index, byte) in block.iter().enumerate() {
            cyphertext[index + (i * 8)] = *byte;
        }
    }
}

fn xtea_encrypt_bytes(plaintext: &[u8; 8], key: &[u32; 4]) -> [u8; 8] {
    let delta: u32 = 0x9E3779B9;
    let mut v0: u32 = u32::from_le_bytes([plaintext[0], plaintext[1], plaintext[2], plaintext[3]]);
    let mut v1: u32 = u32::from_le_bytes([plaintext[4], plaintext[5], plaintext[6], plaintext[7]]);
    let mut sum: u32 = 0;
    for _i in 0..64 {
        v0 = v0.wrapping_add(
            v1.wrapping_add((v1 << 4) ^ (v1 >> 5))
                ^ sum.wrapping_add(key[(sum & 3) as usize] as u32),
        );

        sum = sum.wrapping_add(delta);

        v1 = v1.wrapping_add(
            (v0.wrapping_add(((v0 << 4) ^ (v0 >> 5))))
                ^ sum.wrapping_add(key[((sum >> 11) & 3) as usize] as u32),
        );
    }
    let v0bytes: [u8; 4] = v0.to_le_bytes();
    let v1bytes: [u8; 4] = v1.to_le_bytes();
    let result: [u8; 8] = [
        v0bytes[0], v0bytes[1], v0bytes[2], v0bytes[3], v1bytes[0], v1bytes[1], v1bytes[2],
        v1bytes[3],
    ];

    return result;
}

fn xtea_decrypt_bytes(cyphertext: &[u8; 8], key: &[u32; 4]) -> [u8; 8] {
    let delta: u32 = 0x9E3779B9;
    let mut v0: u32 =
        u32::from_le_bytes([cyphertext[0], cyphertext[1], cyphertext[2], cyphertext[3]]);
    let mut v1: u32 =
        u32::from_le_bytes([cyphertext[4], cyphertext[5], cyphertext[6], cyphertext[7]]);
    let mut sum: u32 = delta.wrapping_mul(64);
    for _i in 0..64 {
        v1 = v1.wrapping_sub(
            v0.wrapping_add(((v0 << 4) ^ (v0 >> 5)))
                ^ sum.wrapping_add(key[((sum >> 11) & 3) as usize] as u32),
        );

        sum = sum.wrapping_sub(delta);

        v0 = v0.wrapping_sub(
            v1.wrapping_add((v1 << 4) ^ (v1 >> 5))
                ^ sum.wrapping_add(key[(sum & 3) as usize] as u32),
        );
    }
    let v0bytes: [u8; 4] = v0.to_le_bytes();
    let v1bytes: [u8; 4] = v1.to_le_bytes();
    let result: [u8; 8] = [
        v0bytes[0], v0bytes[1], v0bytes[2], v0bytes[3], v1bytes[0], v1bytes[1], v1bytes[2],
        v1bytes[3],
    ];

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut text = b"".clone();
        let key = [0_u32; 4];

        // Test Encryption-Direction
        xtea_encrypt(&mut text[..], &key);
        assert_eq!(text, [0_u8; 0]);

        // Test Decryption-Direction
        xtea_decrypt(&mut text[..], &key);
        assert_eq!(text, [0_u8; 0]);
    }

    #[test]
    fn fox() {
        let mut text = b"The quick brown fox jumps over the lazy dog".clone();
        let key = [0_u32, 1_u32, 2_u32, 3_u32];

        // Test Encryption-Direction
        xtea_encrypt(&mut text[..], &key);
        assert_eq!(
            text,
            [
                148, 110, 42, 29, 200, 236, 240, 40, 33, 211, 71, 94, 128, 220, 128, 196, 116, 126,
                86, 53, 154, 253, 134, 19, 174, 36, 42, 46, 230, 1, 169, 3, 54, 243, 75, 171, 49,
                60, 240, 23, 100, 111, 103
            ]
        );

        // Test Decryption-Direction
        xtea_decrypt(&mut text[..], &key);
        assert_eq!(text, b"The quick brown fox jumps over the lazy dog".clone());
    }
}
