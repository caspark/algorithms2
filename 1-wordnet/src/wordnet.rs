struct Synset {
    nouns: Vec<String>,

}

struct WordNet {
    synsets: Vec<Synset>,
    hypernyms: Digraph<String>,
}

impl WordNet {
    pub fn parse(synsetsPath: String, hypernymsPath: String) -> WordNet {
        use std::io::prelude::*;

        let mut file = try!(File::open(filename));
        let mut content = String::new();
        try!(file.read_to_string(&mut content));

        let re = regex!(r"(?P<id>\d+),(?P<nouns>.+),(?P<gloss>.+)");
        let mut points = Vec::new();
        for line in content.split("\n").skip(1) {
            if line.len() == 0 {
                break; // end of file
            }

            let coords = {
                use mdo::option::{bind, ret};
                mdo! {
                    caps =<< re.captures(line).as_ref();
                    x_str =<< caps.name("x");
                    x =<< x_str.parse().ok();
                    y_str =<< caps.name("y");
                    y =<< y_str.parse().ok();
                    ret ret((x, y))
                }
            };
            points.push(coords.expect(&format!("Failed to parse line '{}'", line)));
        }
        Ok(points)
    }

    pub fn nouns() -> Vec<String> {
        panic!("Not implemented");
    }

    pub fn is_noun(word: String) -> Boolean {
        panic!("Not implemented");
    }

    pub fn distance(nounA: String, nounB: String) -> i32 {
        panic!("Not implemented");
    }

    pub fn sap(nounA: String, nounB: String) -> String {
        panic!("Not implemented");
    }
}
