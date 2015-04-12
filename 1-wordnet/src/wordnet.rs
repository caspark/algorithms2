use std::collections::HashMap;
use std::io;
use digraph::Digraph;

struct Synset {
    nouns: Vec<String>,
    gloss: String,
}

impl Synset {
    // Creates a synset with no gloss.
    fn new(nouns: Vec<String>) -> Synset {
        Synset {
            nouns: nouns,
            gloss: "".to_string(),
        }
    }
}

struct WordNet {
    nouns_to_synsets: HashMap<String, usize>, // usize = the synset id
    synsets: Vec<Synset>, // ordered by id; synset with synset id 0 is at position 0
    hypernyms: Digraph, // indexes = the indexes of the synsets
}

impl WordNet {
    pub fn create_by_parsing_files(synsets_path: String, hypernyms_path: String) -> io::Result<WordNet> {
        use std::io::prelude::*;
        use std::fs::File;

        // parse synsets
        let mut synsets_file = try!(File::open(synsets_path));
        let mut synsets_content = String::new();
        try!(synsets_file.read_to_string(&mut synsets_content));

        let re = regex!(r"(?P<id>\d+),(?P<nouns>.+),(?P<gloss>.+)");
        let mut synsets = Vec::new();
        for line in synsets_content.split("\n").skip(1) {
            if line.len() == 0 {
                break; // end of file
            }

            let parsed = {
                use mdo::option::{bind, ret};
                mdo! {
                    caps =<< re.captures(line).as_ref();
                    id_str =<< caps.name("id");
                    id =<< id_str.parse::<usize>().ok();
                    nouns =<< caps.name("nouns");
                    gloss =<< caps.name("gloss");
                    ret ret((id, Synset {
                        nouns: nouns.split(" ").map(|s| s.to_string()).collect::<Vec<_>>(),
                        gloss: gloss.to_string()
                    }))
                }
            };
            if let Some((synset_id, synset)) = parsed {
                synsets.push(synset);
                assert_eq!(synset_id, synsets.len() - 1);
            } else {
                panic!("Failed to parse line '{}'", line)
            }
        }

        // parse hypernyms
        let mut hypernyms_file = try!(File::open(hypernyms_path));
        let mut hypernyms_content = String::new();
        try!(hypernyms_file.read_to_string(&mut hypernyms_content));

        let mut hypernyms_edges = Vec::new();
        for line in hypernyms_content.split("\n").skip(1) {
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
        }

        Ok(WordNet::create_from_synsets_and_hypernyms(synsets, hypernyms_edges))
    }

    fn create_from_synsets_and_hypernyms(synsets: Vec<Synset>, hypernyms_edges: Vec<(i32, i32)>) -> WordNet {
        let mut nouns_to_synsets = HashMap::new();
        for (synset_id, synset) in synsets.iter().enumerate() {
            for noun in synset.nouns.iter() {
                //HACK clone the noun rather than worrying about lifetime constraints
                // (it's inefficient but technically allowed by the instructions)
                nouns_to_synsets.insert(noun.clone(), synset_id);
            }
        }

        let mut hypernyms = Digraph::new(synsets.len() as i32);
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

    pub fn distance(&self, noun_a: &String, noun_b: &String) -> i32 {
        assert!(self.is_noun(noun_a), format!("noun_a of {} is not a known noun!", noun_a));
        assert!(self.is_noun(noun_b), format!("noun_b of {} is not a known noun!", noun_b));

        panic!("Not implemented");
    }

    pub fn sap(&self, noun_a: &String, noun_b: &String) -> Synset {
        assert!(self.is_noun(noun_a), format!("noun_a of {} is not a known noun!", noun_a));
        assert!(self.is_noun(noun_b), format!("noun_b of {} is not a known noun!", noun_b));

        panic!("Not implemented");
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
}
