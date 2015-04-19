use std::borrow::ToOwned;
use std::collections::{HashMap, HashSet};
use std::io;
use digraph::Digraph;
use sap;

#[derive(PartialEq, Eq, Debug)]
pub struct Synset {
    nouns: Vec<String>,
    gloss: String,
}

impl Synset {
    // Creates a synset with no gloss.
    pub fn new(nouns: Vec<String>) -> Synset {
        Synset {
            nouns: nouns,
            gloss: "".to_string(),
        }
    }
}

pub struct WordNet {
    nouns_to_synsets: HashMap<String, HashSet<usize>>, // each usize is the id of a synset which contains this noun
    synsets: Vec<Synset>, // ordered by id; synset with synset id 0 is at position 0
    hypernyms: Digraph, // indexes = the indexes of the synsets
}

impl WordNet {
    pub fn create_by_parsing_files(synsets_path: &String, hypernyms_path: &String) -> io::Result<WordNet> {
        use std::io::prelude::*;
        use std::fs::File;

        // parse synsets
        let mut synsets_file = try!(File::open(synsets_path));
        let mut synsets_content = String::new();
        try!(synsets_file.read_to_string(&mut synsets_content));

        print!("Parsing synsets from {}", synsets_path);
        let mut synsets = Vec::new();
        for (i, line) in synsets_content.split("\n").enumerate() {
            if line.len() == 0 {
                break; // end of file
            }

            let (synset_id, synset) = {
                let columns = &mut line.splitn(3, ",");
                let id = columns.next().expect("synset must have id").parse::<usize>().ok().expect("synset id must be an int");
                let nouns = columns.next().expect("synset must have nouns");
                let gloss = columns.next().unwrap_or("").to_owned();
                (id, Synset {
                        nouns: nouns.split(" ").map(|s| s.to_owned()).collect::<Vec<_>>(),
                        gloss: gloss,
                    }
                )
            };
            synsets.push(synset);
            assert_eq!(synset_id, synsets.len() - 1);

            if i % 1000 == 0 {
                print!(".");
            }
        }
        println!("done!");

        // parse hypernyms
        let mut hypernyms_file = try!(File::open(hypernyms_path));
        let mut hypernyms_content = String::new();
        try!(hypernyms_file.read_to_string(&mut hypernyms_content));

        print!("Parsing hypernyms from {}", hypernyms_path);
        let mut hypernyms_edges = Vec::new();
        for (i, line) in hypernyms_content.split("\n").enumerate() {
            if line.len() == 0 {
                break; // end of file
            }

            let mut synset = None;
            for id_str in line.split(",") {
                let id = id_str.parse().ok().expect("should only be digits in hypernyms file");
                match synset {
                    None => synset = Some(id),
                    Some(synset_id) => hypernyms_edges.push((synset_id, id)),
                }
            }
            if i % 1000 == 0 {
                print!(".");
            }
        }
        println!("done!");

        Ok(WordNet::create_from_synsets_and_hypernyms(synsets, hypernyms_edges))
    }

    pub fn create_from_synsets_and_hypernyms(synsets: Vec<Synset>, hypernyms_edges: Vec<(usize, usize)>) -> WordNet {
        let mut nouns_to_synsets = HashMap::new();
        for (synset_id, synset) in synsets.iter().enumerate() {
            for noun in synset.nouns.iter() {
                if !nouns_to_synsets.contains_key(noun) {
                    //HACK clone the noun rather than worrying about lifetime constraints
                    // (it's inefficient but technically allowed by the instructions)
                    nouns_to_synsets.insert(noun.clone(), HashSet::new());
                }

                nouns_to_synsets.get_mut(noun).expect("set was just added").insert(synset_id);
            }
        }

        let mut hypernyms = Digraph::new(synsets.len());
        for (a, b) in hypernyms_edges {
            hypernyms.add_edge(a, b);
        }

        WordNet {
            nouns_to_synsets: nouns_to_synsets,
            synsets: synsets,
            hypernyms: hypernyms,
        }
    }

    pub fn nouns(&self) -> Vec<&String> {
        let mut all_nouns = Vec::new();
        for synset in self.synsets.iter() {
            for noun in synset.nouns.iter() {
                all_nouns.push(noun);
            }
        }
        all_nouns
    }

    pub fn is_noun(&self, word: &String) -> bool {
        self.nouns_to_synsets.contains_key(word)
    }

    /// Combines the distance and the shortest ancestral path queries of the original assignment spec into 1 function.
    pub fn relationship(&self, noun_a: &String, noun_b: &String) -> (i32, &Synset) {
        let synsets_for_a = self.nouns_to_synsets.get(noun_a).expect(&format!("noun_a of {} is not a known noun!", noun_a));
        let synsets_for_b = self.nouns_to_synsets.get(noun_b).expect(&format!("noun_b of {} is not a known noun!", noun_b));

        let (dist, ancestor_id) = sap::path_stats_between(
            &self.hypernyms,
            synsets_for_a.iter().cloned().collect(),
            synsets_for_b.iter().cloned().collect()
        ).expect("wordnet graph must be connected so there should be a path");
        (dist, self.synsets.get(ancestor_id).expect("found ancestor should be a known synset"))
    }
}

#[cfg(test)]
mod tests {
    use super::{Synset, WordNet};

    #[test]
    fn should_have_no_nouns_initially() {
        let w = WordNet::create_from_synsets_and_hypernyms(Vec::new(), Vec::new());

        assert!(!w.is_noun(&"dog".to_string()));
    }

    #[test]
    fn should_remember_nouns_in_synsets() {
        let w = WordNet::create_from_synsets_and_hypernyms(
            vec!(
                Synset::new(vec!("dog".to_string(), "hound".to_string())),
                Synset::new(vec!("god".to_string()))
            ),
            Vec::new()
        );

        assert!(w.is_noun(&"dog".to_string()));
        assert!(w.is_noun(&"hound".to_string()));
        assert!(w.is_noun(&"god".to_string()));
        assert!(!w.is_noun(&"cat".to_string()));
    }

    #[test]
    fn should_be_able_to_calculate_relationship_between_nouns() {
        let w = WordNet::create_from_synsets_and_hypernyms(
            vec!(
                Synset::new(vec!("mars".to_string(), "ares".to_string())),
                Synset::new(vec!("god".to_string())),
                Synset::new(vec!("zeus".to_string()))
            ),
            vec![
                (0, 1), // mars and ares are gods
                (2, 1), // zeus is a god
            ]
        );

        assert_eq!(w.relationship(&"mars".to_string(), &"zeus".to_string()),
            (2, &Synset::new(vec!("god".to_string()))) );
        assert_eq!(w.relationship(&"zeus".to_string(), &"mars".to_string()),
            (2, &Synset::new(vec!("god".to_string()))) );

        assert_eq!(w.relationship(&"ares".to_string(), &"zeus".to_string()),
            (2, &Synset::new(vec!("god".to_string()))) );

        assert_eq!(w.relationship(&"ares".to_string(), &"god".to_string()),
            (1, &Synset::new(vec!("god".to_string()))) );
    }
}
