/// Encrypts a given text using the Caesar cipher technique.
///
/// In cryptography, a Caesar cipher, also known as Caesar's cipher, the shift cipher, Caesar's code,
/// or Caesar shift, is one of the simplest and most widely known encryption techniques.
/// It is a type of substitution cipher in which each letter in the plaintext is replaced by a letter
/// some fixed number of positions down the alphabet.
///
/// # Arguments
///
/// * `text` - The text to be encrypted.
/// * `rotation` - The number of rotations (shift) to be applied. It should be less than 26.
///
/// # Returns
///
/// Returns the encrypted string.
///
/// # Panics
///
/// Panics if the rotation value is greater than or equal to 26.
///
/// # Examples
///
/// ```
/// let text = "Hello, World!";
/// let rotation = 3;
/// let encrypted = caesar(text, rotation);
/// assert_eq!(encrypted, "Khoor, Zruog!");
/// ```
pub fn caesar(text: &str, rotation: usize) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };

            (((c as u8 - first) + (rotation % 26) as u8) % 26 + first) as char
        } else {
            c
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(caesar("", 13), "");
    }

    #[test]
    fn rot_13() {
        assert_eq!(caesar("rust", 13), "ehfg");
    }

    #[test]
    fn unicode() {
        assert_eq!(caesar("attack at dawn 攻", 5), "fyyfhp fy ifbs 攻");
    }

    #[test]
    fn rotation_within_alphabet_range() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn rotation_at_alphabet_beginning() {
        assert_eq!(caesar("Hello, World!", 0), "Hello, World!");
    }

    #[test]
    fn rotation_at_alphabet_end() {
        assert_eq!(caesar("Hello, World!", 25), "Gdkkn, Vnqkc!");
    }

    #[test]
    fn encryption_and_decryption() {
        let original_text = "The quick brown fox jumps over the lazy dog.";
        let rotation = 5;
        let encrypted_text = caesar(original_text, rotation);

        assert_eq!(caesar(&encrypted_text, 26 - rotation), original_text);
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 10), "");
    }

    #[test]
    fn input_non_alphabetic_characters() {
        assert_eq!(caesar("12345!@#$%", 3), "12345!@#$%");
    }

    #[test]
    fn input_uppercase_letters() {
        assert_eq!(
            caesar("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 1),
            "BCDEFGHIJKLMNOPQRSTUVWXYZA"
        );
    }

    #[test]
    fn input_mixed_case() {
        assert_eq!(caesar("HeLlO WoRlD", 7), "OlSsV DvYsK");
    }

    #[test]
    fn input_with_whitespace() {
        assert_eq!(caesar("Hello, World!", 13), "Uryyb, Jbeyq!");
    }

    #[test]
    fn input_with_special_characters() {
        assert_eq!(
            caesar("Hello!@#$%^&*()_+World", 4),
            "Lipps!@#$%^&*()_+Asvph"
        );
    }

    #[test]
    fn input_with_numbers() {
        assert_eq!(caesar("Abcd1234XYZ", 10), "Klmn1234HIJ");
    }

    #[test]
    fn large_rotation() {
        assert_eq!(caesar("Large rotation", 139), "Ujapn axcjcrxw");
    }

    #[test]
    fn large_rotation_with_big_input() {
        assert_eq!(caesar("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur ac ultrices ante, at gravida ante. Quisque luctus, ligula nec dictum facilisis, elit leo luctus arcu, ut auctor sapien turpis ut mauris. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nulla vel orci sit amet sem efficitur sagittis a quis augue. Donec semper quam tincidunt hendrerit cursus. Duis placerat gravida diam, in interdum purus dapibus in.", 139), "Uxanv rybdv mxuxa brc jvnc, lxwbnlcncda jmryrblrwp nurc. Ldajkrcda jl ducarlnb jwcn, jc pajermj jwcn. Zdrbzdn udlcdb, urpduj wnl mrlcdv ojlrurbrb, nurc unx udlcdb jald, dc jdlcxa bjyrnw cdayrb dc vjdarb. Xalr ejardb wjcxzdn ynwjcrkdb nc vjpwrb mrb yjacdarnwc vxwcnb, wjblncda armrldudb vdb. Wduuj enu xalr brc jvnc bnv noorlrcda bjprccrb j zdrb jdpdn. Mxwnl bnvyna zdjv crwlrmdwc qnwmanarc ldabdb. Mdrb yujlnajc pajermj mrjv, rw rwcnamdv ydadb mjyrkdb rw.");
    }

    #[test]
    fn very_large_rotation_with_big_input() {
        assert_eq!(caesar("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur ac ultrices ante, at gravida ante. Quisque luctus, ligula nec dictum facilisis, elit leo luctus arcu, ut auctor sapien turpis ut mauris. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nulla vel orci sit amet sem efficitur sagittis a quis augue. Donec semper quam tincidunt hendrerit cursus. Duis placerat gravida diam, in interdum purus dapibus in.", 345876), "Jmpck gnqsk bmjmp qgr ykcr, amlqcarcrsp ybgngqagle cjgr. Aspyzgrsp ya sjrpgacq ylrc, yr epytgby ylrc. Osgqosc jsarsq, jgesjy lca bgarsk dyagjgqgq, cjgr jcm jsarsq ypas, sr ysarmp qyngcl rspngq sr kyspgq. Mpag typgsq lyrmosc nclyrgzsq cr kyelgq bgq nyprspgclr kmlrcq, lyqacrsp pgbgasjsq ksq. Lsjjy tcj mpag qgr ykcr qck cddgagrsp qyegrrgq y osgq ysesc. Bmlca qckncp osyk rglagbslr fclbpcpgr aspqsq. Bsgq njyacpyr epytgby bgyk, gl glrcpbsk nspsq byngzsq gl.");
    }
}
