use std::io;
use digraph::Digraph;

struct Synset {
    nouns: Vec<String>,
    gloss: String,
}

struct WordNet {
    synsets: Vec<Synset>,
    hypernyms: Digraph,
}

impl WordNet {
    pub fn parse(synsetsPath: String, hypernymsPath: String) -> io::Result<WordNet> {
        use std::io::prelude::*;
        use std::fs::File;

        // parse synsets
        let mut synsets_file = try!(File::open(synsetsPath));
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
                    id =<< id_str.parse().ok();
                    nouns =<< caps.name("nouns");
                    gloss =<< caps.name("gloss");
                    ret ret((id, Synset {
                        nouns: nouns.split(" ").map(|s| s.to_string()).collect::<Vec<_>>(),
                        gloss: gloss.to_string()
                    }))
                }
            };
            if let Some((id, synset)) = parsed {
                synsets.push(synset);
                assert_eq!(id, synsets.len() - 1);
            } else {
                panic!("Failed to parse line '{}'", line)
            }
        }

        // parse hypernyms
        let mut hypernyms_file = try!(File::open(hypernymsPath));
        let mut hypernyms_content = String::new();
        try!(hypernyms_file.read_to_string(&mut hypernyms_content));

        let mut hypernyms = Digraph::new(synsets.len() as i32);
        for line in hypernyms_content.split("\n").skip(1) {
            if line.len() == 0 {
                break; // end of file
            }

            let mut synset = None;
            for id_str in line.split(",") {
                let id = id_str.parse().ok().expect("should only be digits in hypernyms file");
                match synset {
                    Some(synset_id) => hypernyms.add_edge(synset_id, id),
                    None => synset = Some(id),
                }
            }
        }

        Ok(WordNet {
            synsets: synsets,
            hypernyms: hypernyms,
        })
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

    pub fn is_noun(word: String) -> bool {
        panic!("Not implemented");
    }

    pub fn distance(noun_a: String, noun_b: String) -> i32 {
        panic!("Not implemented");
    }

    pub fn sap(noun_a: String, noun_b: String) -> String {
        panic!("Not implemented");
    }
}
