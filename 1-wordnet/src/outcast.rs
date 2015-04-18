use wordnet::WordNet;

/// Find which of the given nouns is least related to any other nouns according to the given wordnet.
fn find_outcast<'n>(wordnet: &WordNet, nouns: &'n Vec<String>) -> &'n String {
    nouns.iter().map(|noun|
        (
            noun,
            nouns.iter().map(|other_noun|
                if noun != other_noun {
                    wordnet.relationship(&noun, &other_noun).0
                } else {
                    0
                }
            ).sum::<i32>()
        )
    ).max_by(|&(_, dist_from_other_nouns)| dist_from_other_nouns).expect("nouns must not be empty").0
}

#[cfg(test)]
mod tests {
    use wordnet::{Synset, WordNet};
    use super::find_outcast;

    #[test]
    fn calculates_sample_outcast_correctly() {
        let w = WordNet::create_from_synsets_and_hypernyms(
            vec!(
                Synset::new(vec!("mars".to_string(), "ares".to_string())),
                Synset::new(vec!("god".to_string())),
                Synset::new(vec!("zeus".to_string())),
                Synset::new(vec!("being".to_string())),
                Synset::new(vec!("caspar".to_string()))
            ),
            vec![
                (0, 1), // mars and ares are gods
                (2, 1), // zeus is a god
                (1, 3), // a god is a being
                (4, 3), // caspar is a being
            ]
        );

        assert_eq!(
            find_outcast(&w, &vec!("mars".to_string(), "zeus".to_string(), "god".to_string(), "caspar".to_string())),
            &"caspar".to_string()
        );
    }
}
