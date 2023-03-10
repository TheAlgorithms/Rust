pub trait Hasher<const DIGEST_BYTES: usize> {
    /// return a new instance with default parameters
    fn new_default() -> Self;

    /// Add new data
    fn update(&mut self, data: &[u8]);

    /// Returns the hash of current data. If it is necessary does finalization
    /// work on the instance, thus it may no longer make sense to do `update`
    /// after calling this.
    fn get_hash(&mut self) -> [u8; DIGEST_BYTES];
}

/// HMAC based on RFC2104, applicable to many cryptographic hash functions
pub struct HMAC<const KEY_BYTES: usize, const DIGEST_BYTES: usize, H: Hasher<DIGEST_BYTES>> {
    pub inner_internal_state: H,
    pub outer_internal_state: H,
}

impl<const KEY_BYTES: usize, const DIGEST_BYTES: usize, H: Hasher<DIGEST_BYTES>>
    HMAC<KEY_BYTES, DIGEST_BYTES, H>
{
    pub fn new_default() -> Self {
        HMAC {
            inner_internal_state: H::new_default(),
            outer_internal_state: H::new_default(),
        }
    }

    /// Note that `key` must be no longer than `KEY_BYTES`. According to RFC,
    /// if it is so, you should replace it with its hash. We do not do this
    /// automatically due to fear of `DIGEST_BYTES` not being the same as
    /// `KEY_BYTES` or even being longer than it
    pub fn add_key(&mut self, key: &[u8]) -> Result<(), &'static str> {
        match key.len().cmp(&KEY_BYTES) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                let mut tmp_key = [0u8; KEY_BYTES];
                for (d, s) in tmp_key.iter_mut().zip(key.iter()) {
                    *d = *s;
                }
                // key ^ 0x363636.. should be used as inner key
                for b in tmp_key.iter_mut() {
                    *b ^= 0x36;
                }
                self.inner_internal_state.update(&tmp_key);
                // key ^ 0x5c5c5c.. should be used as outer key, but the key is
                // already XORed with 0x363636.. , so it must now be XORed with
                // 0x6a6a6a..
                for b in tmp_key.iter_mut() {
                    *b ^= 0x6a;
                }
                self.outer_internal_state.update(&tmp_key);
                Ok(())
            }
            _ => Err("Key is longer than `KEY_BYTES`."),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner_internal_state.update(data);
    }

    pub fn finalize(&mut self) -> [u8; DIGEST_BYTES] {
        self.outer_internal_state
            .update(&self.inner_internal_state.get_hash());
        self.outer_internal_state.get_hash()
    }
}

#[cfg(test)]
mod tests {
    use super::super::sha256::tests::get_hash_string;
    use super::super::SHA256;
    use super::HMAC;

    #[test]
    fn sha256_basic() {
        // To test this, use the following command on linux:
        // echo -n "Hello World" | openssl sha256 -hex -mac HMAC -macopt hexkey:"deadbeef"
        let mut hmac: HMAC<64, 32, SHA256> = HMAC::new_default();
        hmac.add_key(&[0xde, 0xad, 0xbe, 0xef]).unwrap();
        hmac.update(b"Hello World");
        let hash = hmac.finalize();
        assert_eq!(
            get_hash_string(&hash),
            "f585fc4536e8e7f378437465b65b6c2eb79036409b18a7d28b6d4c46d3a156f8"
        );
    }
}
