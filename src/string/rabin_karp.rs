pub fn search(pat: &str, txt: &str, q: i32, res: &mut Vec<u32>) {
    let m = pat.len();
    let n = txt.len();
    let mut p = 0 as i32;
    let mut t = 0 as i32;
    let mut h = 1;
    let d = 256;

    if m == 0 && n != 0 {
        res.push(0)
    }

    if m != 0 && n != 0 {
        for _i in 0..m - 1 {
            h = (h * d) % q;
        }

        for i in 0..m {
            p = (d * p + (get_str_char(&pat, &i) as i32)) % q;
            t = (d * t + (get_str_char(&txt, &i) as i32)) % q;
        }

        for i in 0..(n - m + 1) {
            if p == t {
                for j in 0..m {
                    if get_str_char(&txt, &(i + j)) != get_str_char(&pat, &j) {
                        break;
                    }
                    if j == m - 1 {
                        res.push(i as u32);
                    }
                }
            }
            if i < n - m {
                t = (d * (t - (get_str_char(&txt, &i) as i32) * h)
                    + (get_str_char(&txt, &(i + m)) as i32))
                    % q;

                if t < 0 {
                    t = t + q
                }
            }
        }
    }
}

fn get_str_char(s: &str, index: &usize) -> char {
    return s.chars().nth(*index).unwrap() as char;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let txt = "hello world hello";
        let pat = "hello";
        let mut indexes = vec![];
        search(&pat, &txt, 101, &mut indexes);
        assert_eq!(indexes, [0, 12]);
    }

    #[test]
    fn case_2() {
        let txt = "AABAACAADAABAABA";
        let pat = "AABA";
        let mut indexes = vec![];
        search(&pat, &txt, 101, &mut indexes);
        assert_eq!(indexes, [0, 9, 12]);
    }

    #[test]
    fn case_3() {
        let txt = "A";
        let pat = "A";
        let mut indexes = vec![];
        search(&pat, &txt, 101, &mut indexes);
        assert_eq!(indexes, [0]);
    }

    #[test]
    fn both_empty() {
        let txt = "";
        let pat = "";
        let mut indexes = vec![];
        search(&pat, &txt, 101, &mut indexes);
        assert_eq!(indexes, []);
    }

    #[test]
    fn one_empty() {
        let txt = "hello";
        let pat = "";
        let mut indexes_1 = vec![];
        let mut indexes_2 = vec![];
        search(&pat, &txt, 101, &mut indexes_1);
        search(&txt ,&pat, 101, &mut indexes_2);
        assert_eq!(indexes_1, [0]);
        assert_eq!(indexes_2, []);
    }
}
