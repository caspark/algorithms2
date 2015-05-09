use std::collections::HashSet;
use trie::{Presence, Trie};

#[derive(Debug)]
pub struct BoggleBoard {
    width: usize,
    height: usize,
    letters: Vec<u8>, // no unicode support needed
}

impl BoggleBoard {
    pub fn new(width: usize, height: usize, letters: Vec<u8>) -> BoggleBoard {
        assert_eq!(width * height, letters.len());
        BoggleBoard {
            width: width,
            height: height,
            letters: letters,
        }
    }
}

#[derive(Debug)]
pub struct BoggleSolver {
    words: Trie,
}

impl BoggleSolver {
    pub fn new(valid_words: &[String]) -> BoggleSolver {
        BoggleSolver {
            words: {
                let mut trie = Trie::new();
                for word in valid_words {
                    trie.add(word.as_bytes());
                }
                trie
            }
        }
    }

    pub fn find_valid_words<'s>(&self, board: &BoggleBoard) -> Vec<String> {
        let max_word_len = board.width * board.height; // aka the final position in the board

        let to_word = |path_so_far: &[usize]| {
            let mut built_word = Vec::with_capacity(max_word_len);
            for i in path_so_far {
                built_word.push(board.letters[*i])
            }
            built_word
        };

        let mut search_stack = (0..max_word_len).map(|i| vec![i]).collect::<Vec<_>>();
        let mut found_words = Vec::new();


        while search_stack.len() > 0 {
            let path_so_far = search_stack.pop().expect("search stack known to be non-empty");

            let word = to_word(&path_so_far[..]);

            let continue_with_path = path_so_far.len() < max_word_len &&
                match self.words.contains(&word[..]) {
                    Presence::Missing => false,
                    Presence::Prefix => true,
                    Presence::Present => {
                        found_words.push(String::from_utf8(word).unwrap());
                        true
                    },
                };

            if continue_with_path {
                let latest_pos: usize= path_so_far[path_so_far.len()];

                let (on_left_edge, on_right_edge) = {
                    let mod_result = latest_pos % board.width;
                    (mod_result == 0, mod_result == board.width - 1)
                };
                let (on_top_edge, on_bottom_edge) = {
                    (latest_pos - board.width < 0, latest_pos + board.width >= max_word_len)
                };

                let mut consider_path = |path_so_far: &Vec<usize>, candidate_pos| {
                    if !path_so_far.contains(&candidate_pos) {
                        let mut path_to_consider = path_so_far.clone();
                        path_to_consider.push(candidate_pos);
                        search_stack.push(path_to_consider);
                    }
                };

                // consider up to 8 surrounding position
                if !on_top_edge {
                    if !on_left_edge {
                        consider_path(&path_so_far, latest_pos - board.width - 1);
                    }
                    consider_path(&path_so_far, latest_pos - board.width);
                    if !on_right_edge {
                        consider_path(&path_so_far, latest_pos - board.width + 1);
                    }
                }
                if !on_left_edge {
                    consider_path(&path_so_far, latest_pos - 1);
                }
                // (don't consider the 'middle' position, because we already include it)
                if !on_right_edge {
                    consider_path(&path_so_far, latest_pos + 1);
                }
                if !on_bottom_edge {
                    if !on_left_edge {
                        consider_path(&path_so_far, latest_pos + board.width - 1);
                    }
                    consider_path(&path_so_far, latest_pos + board.width);
                    if !on_right_edge {
                        consider_path(&path_so_far, latest_pos + board.width + 1);
                    }
                }
            }
       }

        found_words
    }

    pub fn score_of_word(word: &String) -> i32 {
        match word.len() {
            0 | 1 | 2 => 0,
            3 | 4 => 1,
            5 => 2,
            6 => 3,
            7 => 5,
            _ => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BoggleBoard, BoggleSolver};

    #[test]
    fn can_solve_simple_board() {
        let board = BoggleBoard::new(2, 2, vec!['B' as u8, 'A' as u8, 'C' as u8, 'G' as u8]);
        let solver = BoggleSolver::new(&vec!["BAG", "CAB", "BOB", "GAG"].iter().map(|s| s.to_string()).collect::<Vec<_>>()[..]);

        let words = solver.find_valid_words(&board);
        assert_eq!(words, vec!["BAG".to_string(), "CAB".to_string()])
    }

}
