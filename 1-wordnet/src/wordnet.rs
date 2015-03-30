use std::fs::File;
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

        let mut file = try!(File::open(synsetsPath));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));

        let re = regex!(r"(?P<id>\d+),(?P<nouns>.+),(?P<gloss>.+)");
        let mut synsets = Vec::new();
        for line in content.split("\n").skip(1) {
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

        let hypernyms = Digraph::new(synsets.len() as i32);

        //TODO parse hypernym graph

        Ok(WordNet {
            synsets: synsets,
            hypernyms: hypernyms,
        })
    }

    pub fn nouns() -> Vec<String> {
        panic!("Not implemented");
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
