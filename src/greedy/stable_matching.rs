use std::collections::{HashMap, VecDeque};

pub fn stable_matching(
    men_preferences: &HashMap<String, Vec<String>>,
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    let mut free_men: VecDeque<String> = VecDeque::new();
    let mut current_partner: HashMap<String, Option<String>> = HashMap::new();
    let mut man_engaged: HashMap<String, Option<String>> = HashMap::new();
    let mut next_proposal: HashMap<String, usize> = HashMap::new();

    for man in men_preferences.keys() {
        free_men.push_back(man.clone());
        next_proposal.insert(man.clone(), 0);
    }

    for woman in women_preferences.keys() {
        current_partner.insert(woman.clone(), None);
    }

    fn woman_prefers_new_man(
        woman: &str,
        man1: &str,
        man2: &str,
        preferences: &HashMap<String, Vec<String>>,
    ) -> bool {
        let woman_preferences = &preferences[woman];
        woman_preferences.iter().position(|m| m == man1).unwrap()
            < woman_preferences.iter().position(|m| m == man2).unwrap()
    }

    while let Some(man) = free_men.pop_front() {
        let man_pref_list = &men_preferences[&man];
        let next_woman_idx = *next_proposal.get(&man).unwrap();
        let woman = &man_pref_list[next_woman_idx];

        next_proposal.insert(man.clone(), next_woman_idx + 1);

        if let Some(current_man) = current_partner[woman].clone() {
            if woman_prefers_new_man(woman, &man, &current_man, women_preferences) {
                man_engaged.insert(man.clone(), Some(woman.clone()));
                current_partner.insert(woman.clone(), Some(man.clone()));
                free_men.push_back(current_man);
            } else {
                free_men.push_back(man);
            }
        } else {
            man_engaged.insert(man.clone(), Some(woman.clone()));
            current_partner.insert(woman.clone(), Some(man.clone()));
        }
    }

    let mut stable_matches: HashMap<String, String> = HashMap::new();
    for (man, woman_option) in man_engaged {
        if let Some(woman) = woman_option {
            stable_matches.insert(man, woman);
        }
    }

    stable_matches
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_stable_matching_scenario_1() {
        let men_preferences = HashMap::from([
            (
                "A".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "B".to_string(),
                vec!["Y".to_string(), "X".to_string(), "Z".to_string()],
            ),
            (
                "C".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
        ]);

        let women_preferences = HashMap::from([
            (
                "X".to_string(),
                vec!["B".to_string(), "A".to_string(), "C".to_string()],
            ),
            (
                "Y".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
            (
                "Z".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);

        let expected_matches1 = HashMap::from([
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        let expected_matches2 = HashMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        assert!(matches == expected_matches1 || matches == expected_matches2);
    }

    #[test]
    fn test_stable_matching_empty() {
        let men_preferences = HashMap::new();
        let women_preferences = HashMap::new();

        let matches = stable_matching(&men_preferences, &women_preferences);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_stable_matching_duplicate_preferences() {
        let men_preferences = HashMap::from([
            ("A".to_string(), vec!["X".to_string(), "X".to_string()]), // Man with duplicate preferences
            ("B".to_string(), vec!["Y".to_string()]),
        ]);

        let women_preferences = HashMap::from([
            ("X".to_string(), vec!["A".to_string(), "B".to_string()]),
            ("Y".to_string(), vec!["B".to_string()]),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);
        let expected_matches = HashMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
        ]);

        assert_eq!(matches, expected_matches);
    }

    #[test]
    fn test_stable_matching_single_pair() {
        let men_preferences = HashMap::from([("A".to_string(), vec!["X".to_string()])]);
        let women_preferences = HashMap::from([("X".to_string(), vec!["A".to_string()])]);

        let matches = stable_matching(&men_preferences, &women_preferences);
        let expected_matches = HashMap::from([("A".to_string(), "X".to_string())]);

        assert_eq!(matches, expected_matches);
    }
}
