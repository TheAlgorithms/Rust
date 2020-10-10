use std::collections::HashMap;

/// Takes in a byte array and returns it as a base64 string
pub fn base64_encode(in_str: &[u8]) -> Result<String, &'static str> {
    let alphanum = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let mut res: Vec<u8> = Vec::new();
    let mut i = 0;

    if in_str.len() == 0 {
        return Ok("".to_string());
    }

    loop {
        // All but the 2 lowest bits. Want the value as if they were the lowest ones
        res.push(alphanum[((in_str[i + 0] & 0xfc) >> 2) as usize]);

        if i + 1 >= in_str.len() {
            res.push(alphanum[(((in_str[i + 0] & 3) << 4) | 0) as usize]);
            res.push('=' as u8);
            res.push('=' as u8);
            break;
        }

        // 2 Lowest bits from the 1st byte, and the 4 highest from the 2nd.
        res.push(alphanum[(((in_str[i + 0] & 3) << 4) | (in_str[i + 1] & 0xf0) >> 4) as usize]);

        if i + 2 >= in_str.len() {
            res.push(alphanum[(((in_str[i + 1] & 0xf) << 2) | 0) as usize]);
            res.push('=' as u8);
            break;
        }

        // 4 Lowest bits from the 2nd byte, and the 2 highest from the 3rd.
        res.push(alphanum[(((in_str[i + 1] & 0xf) << 2) | (in_str[i + 2] & 0xc0) >> 6) as usize]);

        // 6 lowest bits from the 3rd byte
        res.push(alphanum[(in_str[i + 2] & 0x3f) as usize]);

        i += 3;

        if i >= in_str.len() {
            break;
        }
    }

    Ok(String::from_utf8_lossy(&res).into_owned())
}

/// Takes in a String and returns the raw bytes as a Vec<u8>
pub fn base64_decode(in_str: &str) -> Result<Vec<u8>, &'static str> {
    let mut resvec = Vec::new();
    let mut tmpvec = Vec::new();
    let alphanum = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars();
    let mut mapping = HashMap::new();

    if in_str.len() % 4 != 0 {
        return Err("invalid length");
    }

    for (idx, val) in alphanum.enumerate() {
        mapping.insert(val, idx);
    }

    for c in in_str.chars() {
        if mapping.contains_key(&c) {
            tmpvec.push(mapping[&c] as u8);
        } else if c == '=' {
            break;
        } else {
            return Err("invalid character");
        }
    }

    let mut tmp_1 = 0 as u8;
    let mut tmp_2 = 0 as u8;
    let mut tmp_3 = 0 as u8;
    for (idx, val) in tmpvec.iter().enumerate() {
        let calc = idx % 4;

        match calc {
            0 => {
                tmp_1 = val << 2;
            }
            1 => {
                // Grab the 2 highest bits (of the 6 bit character). shift them down to the lowest 2 bits
                tmp_1 = tmp_1 | (val & 0x30) >> 4;
                resvec.push(tmp_1); // this byte is done now

                // the next byte gets its top 4 bits from this chars lowest 4 bits.
                tmp_2 = (val & 0xf) << 4;
            }
            2 => {
                // Grab the 4 highest bits of the char and put them at the lowest 4 bits
                tmp_2 = tmp_2 | (val & 0x3c) >> 2;
                resvec.push(tmp_2);
                // Grab the 2 lowest bits of the char and shift them to the 2 highest bits
                tmp_3 = (val & 0x3) << 6;
            }
            3 => {
                // Grab (all) 6 lowest bits of the char and put them in at the 6 lowest bits
                tmp_3 = tmp_3 | val & 0x3f;

                resvec.push(tmp_3);
            }
            _ => {
                panic!("Not possible");
            }
        }
    }

    return Ok(resvec);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_1234() {
        assert_eq!(base64_encode(b"1234"), Ok("MTIzNA==".to_string()));
    }

    #[test]
    fn test_decode_1234() {
        assert_eq!(base64_decode("MTIzNA=="), Ok(b"1234".to_vec()));
    }

    #[test]
    fn test_invalid_length() {
        assert_eq!(base64_decode("12356"), Err("invalid length"));
    }

    #[test]
    fn test_invalid_character() {
        assert_eq!(base64_decode("123_"), Err("invalid character"));
    }

    #[test]
    fn test_zero_length() {
        assert_eq!(base64_encode(b""), Ok("".to_string()));
    }
}
