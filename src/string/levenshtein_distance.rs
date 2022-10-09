pub fn levenshtein_distance(string1: &str, string2: &str) -> usize {
    if string1.is_empty() {
        return string2.len();
    }

    let mut d = Vec::with_capacity(string1.len());
    for i in 0..=string1.len() {
        d.push(i);
    }

    let mut j = 1;
    for c2 in string2.chars() {
        let mut previous_substitution_cost = d[0];
        d[0] = j;

        let mut i = 1;
        for c1 in string1.chars() {
            let deletion_cost = d[i - 1] + 1;
            let insertion_cost = d[i] + 1;
            let substitution_cost = previous_substitution_cost + if c1 == c2 { 0 } else { 1 };

            previous_substitution_cost = d[i];
            d[i] = min3(deletion_cost, insertion_cost, substitution_cost);

            i = i + 1;
        }

        j = j + 1;
    }

    d[d.len() - 1]
}

#[cfg(test)]
mod levenshtein_distance_should {
    use super::levenshtein_distance;

    #[test]
    fn return_0_with_empty_strings() {
        assert_eq!(0, levenshtein_distance("", ""));
    }

    #[test]
    fn return_1_with_empty_and_a() {
        assert_eq!(1, levenshtein_distance("", "a"));
    }

    #[test]
    fn return_1_with_a_and_empty() {
        assert_eq!(1, levenshtein_distance("a", ""));
    }

    #[test]
    fn return_1_with_ab_and_a() {
        assert_eq!(1, levenshtein_distance("ab", "a"));
    }

    #[test]
    fn return_0_with_foobar_and_foobar() {
        assert_eq!(0, levenshtein_distance("foobar", "foobar"));
    }

    #[test]
    fn return_6_with_foobar_and_barfoo() {
        assert_eq!(6, levenshtein_distance("foobar", "barfoo"));
    }

    #[test]
    fn return_1_with_kind_and_bind() {
        assert_eq!(1, levenshtein_distance("kind", "bind"));
    }

    #[test]
    fn return_3_with_winner_and_win() {
        assert_eq!(3, levenshtein_distance("winner", "win"));
    }
}

fn min3(a: usize, b: usize, c: usize) -> usize {
    if a < b
    {
        if c < a { c } else { a }
    }
    else
    {
        if c < b { c } else { b }
    }
}

#[cfg(test)]
mod min3_should {
    use super::min3;

    #[test]
    fn return_1_with_1_2_3() {
        assert_eq!(1, min3(1, 2, 3));
    }

    #[test]
    fn return_1_with_3_2_1() {
        assert_eq!(1, min3(3, 2, 1));
    }

    #[test]
    fn return_1_with_2_3_1() {
        assert_eq!(1, min3(2, 3, 1));
    }
}