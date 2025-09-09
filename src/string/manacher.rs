pub fn manacher(s: String) -> String {
    let l = s.len();
    if l <= 1 {
        return s;
    }

    // 1. Preprocessing: insert separators
    let mut chars: Vec<char> = Vec::with_capacity(s.len() * 2 + 1);
    for c in s.chars() {
        chars.push('#');
        chars.push(c);
    }
    chars.push('#');
    let n = chars.len();

    // 2. p[i] represents the radius—how far it can expand symmetrically from the center in both directions.
    let mut p = vec![0usize; n];
    let mut center = 0;
    let mut right = 0;

    for i in 0..n {
        // Mirror position
        let mirror = 2 * center as isize - i as isize;
        // Inherit the value from the mirror
        if i < right {
            if mirror >= 0 {
                p[i] = p[mirror as usize].min(right - i);
            } else {
                p[i] = 0;
            }
        }

        // Expand
        while i + p[i] + 1 < n && i > p[i] && chars[i + p[i] + 1] == chars[i - p[i] - 1] {
            p[i] += 1;
        }

        // Update the center and the right boundary
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }

    // 3. Find the maximum

    let (center_of_max, &radius_of_max) = p.iter().enumerate().max_by_key(|&(_, &x)| x).unwrap();

    // 4. Construct the answer
    let start = center_of_max - radius_of_max;
    let end = center_of_max + radius_of_max;
    let answer: String = chars[start..=end]
        .iter()
        .filter(|&&c| c != '#')
        .cloned()
        .collect();
    answer.replace('#', "")
}

#[cfg(test)]
mod tests {
    use super::manacher;

    #[test]
    fn get_longest_palindrome_by_manacher() {
        assert_eq!(manacher("babad".to_string()), "aba".to_string());
        assert_eq!(manacher("cbbd".to_string()), "bb".to_string());
        assert_eq!(manacher("a".to_string()), "a".to_string());

        let ac_ans = manacher("ac".to_string());
        assert!(ac_ans == *"a" || ac_ans == *"c");
    }
}
