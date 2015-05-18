use std::collections::HashSet;
use trie::{Presence, Trie};

/// because our Trie only supports storing up to 26 characters, we need to make sure that letters are stored
/// as values from 0 to 25
const LETTER_OFFSET: u8 = 65;
/// the letter Q after applying offset
const OFFSET_Q: u8 = ('Q' as i64 - LETTER_OFFSET as i64) as u8;
/// the letter U after applying offset
const OFFSET_U: u8 = ('U' as i64 - LETTER_OFFSET as i64) as u8;

#[derive(Debug)]
pub struct BoggleBoard {
    width: usize,
    height: usize,
    letters: Vec<u8>, // no unicode support needed
}

impl BoggleBoard {
    pub fn new(width: usize, height: usize, letters: Vec<u8>) -> BoggleBoard {
        assert_eq!(width * height, letters.len());

        let adjusted_letters = letters.iter().map(|l| (*l as i32 - LETTER_OFFSET as i32) as u8).collect::<Vec<_>>();
        drop(letters);

        assert!(!adjusted_letters.iter().any(|l| *l >= LETTER_OFFSET), "board has letters outside expected range!");
        BoggleBoard {
            width: width,
            height: height,
            letters: adjusted_letters,
        }
    }
}

#[derive(Debug)]
pub struct BoggleSolver {
    words: Trie,
}

impl BoggleSolver {
    pub fn new(valid_words: Vec<String>) -> BoggleSolver {
        BoggleSolver {
            words: {
                let mut trie = Trie::new();
                for word in valid_words {
                    let letters = word.as_bytes().iter()
                        .map(|l| (*l as i32 - LETTER_OFFSET as i32) as u8)
                        .collect::<Vec<_>>();

                    trie.add(&letters[..]);
                }
                trie
            }
        }
    }

    pub fn find_valid_words<'s>(&self, board: &BoggleBoard) -> HashSet<String> {
        let max_word_len = board.width * board.height; // aka the final position in the board
        let mut built_word = Vec::with_capacity(max_word_len);

        let mut search_stack = (0..max_word_len).map(|i| vec![i]).collect::<Vec<_>>();
        let mut found_words = HashSet::new();

        while search_stack.len() > 0 {
            let path_so_far = search_stack.pop().expect("search stack known to be non-empty");

            built_word.truncate(0);
            for i in path_so_far.iter() {
                let letter = board.letters[*i];
                built_word.push(letter);
                if letter == OFFSET_Q {
                    built_word.push(OFFSET_U);
                }
            }

            let path_is_possible_word = match self.words.contains(&built_word[..]) {
                    Presence::Missing => false,
                    Presence::Prefix => true,
                    Presence::Present => {
                        found_words.insert(String::from_utf8(built_word.iter().map(|l| l + LETTER_OFFSET).collect()).unwrap());
                        true
                    },
                };

            if path_so_far.len() < max_word_len && path_is_possible_word {
                let latest_pos: usize = path_so_far[path_so_far.len() - 1];

                let (on_left_edge, on_right_edge) = {
                    let mod_result = latest_pos % board.width;
                    (mod_result == 0, mod_result == board.width - 1)
                };
                let (on_top_edge, on_bottom_edge) = (
                    latest_pos < board.width,
                    latest_pos + board.width >= max_word_len
                );

                let mut consider_path = |path_so_far: &Vec<usize>, candidate_pos| {
                    if !path_so_far.contains(&candidate_pos) {
                        let mut path_to_consider = path_so_far.clone();
                        path_to_consider.push(candidate_pos);
                        search_stack.push(path_to_consider);
                    }
                };

                // consider up to 8 surrounding positions
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
            _ => 11,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BoggleBoard, BoggleSolver, LETTER_OFFSET, OFFSET_Q, OFFSET_U};

    #[test]
    fn letter_offsets_are_correct() {
        assert_eq!(LETTER_OFFSET as char, 'A');
        assert_eq!(OFFSET_Q as i64 + LETTER_OFFSET as i64, 'Q' as i64);
        assert_eq!(OFFSET_U as i64 + LETTER_OFFSET as i64, 'U' as i64);
    }

    #[test]
    fn can_solve_simple_board() {
        let board = BoggleBoard::new(2, 2, vec!['B' as u8, 'A' as u8, 'C' as u8, 'G' as u8]);
        let solver = BoggleSolver::new(vec!["BAG", "CAB", "BOB", "GAG"].iter().map(|s| s.to_string()).collect::<Vec<_>>());

        let mut words = solver.find_valid_words(&board).iter().cloned().collect::<Vec<_>>();
        &words[..].sort();
        assert_eq!(words, vec!["BAG".to_string(), "CAB".to_string()])
    }

    #[test]
    fn handles_qu_correctly() {
        let board = BoggleBoard::new(2, 2, vec!['Q' as u8, 'I' as u8, 'T' as u8, 'X' as u8]);
        let solver = BoggleSolver::new(vec!["QUIT", "QX"].iter().map(|s| s.to_string()).collect::<Vec<_>>());

        let words = solver.find_valid_words(&board).iter().cloned().collect::<Vec<_>>();

        // Q on board should be equal to QU, and we shouldn't be able to use Q without U.
        assert_eq!(words, vec!["QUIT".to_string()])
    }
}
