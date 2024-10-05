use std::collections::{BTreeMap, VecDeque};

// This function performs the Gale-Shapley stable matching algorithm
pub fn stable_matching(
    men_preferences: &BTreeMap<String, Vec<String>>,
    women_preferences: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, String> {
    // Free men: those who haven't been paired yet
    let mut free_men: VecDeque<String> = VecDeque::new();

    // Holds the current partner of each woman (if any)
    let mut current_partner: BTreeMap<String, Option<String>> = BTreeMap::new();

    // Holds the current engagement of each man (if any)
    let mut man_engaged: BTreeMap<String, Option<String>> = BTreeMap::new();

    // Holds the position in each man's preference list they are currently proposing to
    let mut next_proposal: BTreeMap<String, usize> = BTreeMap::new();

    // Initialize all men as free
    for man in men_preferences.keys() {
        free_men.push_back(man.clone());
        next_proposal.insert(man.clone(), 0);
    }

    // Initialize all women as having no partner
    for woman in women_preferences.keys() {
        current_partner.insert(woman.clone(), None);
    }

    // Helper function to check if a woman prefers a new man over her current partner
    fn woman_prefers_new_man(
        woman: &str,
        man1: &str,
        man2: &str,
        preferences: &BTreeMap<String, Vec<String>>,
    ) -> bool {
        let woman_preferences = &preferences[woman];
        woman_preferences.iter().position(|m| m == man1).unwrap()
            < woman_preferences.iter().position(|m| m == man2).unwrap()
    }

    // While there are free men
    while let Some(man) = free_men.pop_front() {
        // Get the man's preference list and find the next woman to propose to
        let man_pref_list = &men_preferences[&man];
        let next_woman_idx = *next_proposal.get(&man).unwrap();
        let woman = &man_pref_list[next_woman_idx];

        // Move to the next proposal for this man
        next_proposal.insert(man.clone(), next_woman_idx + 1);

        if let Some(current_man) = current_partner[woman].clone() {
            // The woman is already engaged, compare her preference
            if woman_prefers_new_man(woman, &man, &current_man, women_preferences) {
                // Woman prefers the new man, engage them
                man_engaged.insert(man.clone(), Some(woman.clone()));
                current_partner.insert(woman.clone(), Some(man.clone()));
                free_men.push_back(current_man); // The old partner becomes free
            } else {
                // Woman prefers her current partner, so the man remains free
                free_men.push_back(man);
            }
        } else {
            // The woman is not engaged, engage her with the man
            man_engaged.insert(man.clone(), Some(woman.clone()));
            current_partner.insert(woman.clone(), Some(man.clone()));
        }
    }

    // Return the final stable matches as a BTreeMap
    let mut stable_matches: BTreeMap<String, String> = BTreeMap::new();
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

    #[test]
    fn test_stable_matching() {
        // Test 1

        let men_preferences = BTreeMap::from([
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

        let women_preferences = BTreeMap::from([
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

        // Expected stable matching can be one of two possible outcomes
        let expected_matches1 = BTreeMap::from([
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        let expected_matches2 = BTreeMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        // Assert that the result matches one of the expected outcomes
        assert!(
            matches == expected_matches1 || matches == expected_matches2,
            "Matching result is not as expected"
        );

        //Test 2: Empty Inputs

        let men_preferences = BTreeMap::from([]);
        let women_preferences = BTreeMap::from([]);
        let matches = stable_matching(&men_preferences, &women_preferences);
        let expected_matches1 = BTreeMap::from([]);
        let expected_matches2 = BTreeMap::from([]);

        // Assert that the result matches one of the expected outcomes
        assert!(
            matches == expected_matches1 || matches == expected_matches2,
            "Matching result is not as expected"
        );
    }
}
