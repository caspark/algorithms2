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

        let mut found_words = HashSet::new();
        let mut built_word: Vec<u8> = Vec::with_capacity(max_word_len);
        let mut path_so_far: Vec<usize> = Vec::with_capacity(max_word_len);
        for next_pos in (0..max_word_len) {
            self.find_valid_words_inner(board, max_word_len, &mut found_words, &mut built_word,
                &mut path_so_far, next_pos);
        }
        found_words
    }

    //TODO it'd be nicer to write this in non-recursive fashion. next_pos is the only thing that
    // changes in each recursive call.
    fn find_valid_words_inner(&self, board: &BoggleBoard, max_word_len: usize,
                              found_words: &mut HashSet<String>, built_word: &mut Vec<u8>,
                              path_so_far: &mut Vec<usize>, next_pos: usize) {
        path_so_far.push(next_pos);
        let next_char = board.letters[next_pos];
        built_word.push(next_char);
        if next_char == OFFSET_Q {
            built_word.push(OFFSET_U);
        }

        //TODO there's a straightforward optimization possible here: track the current position in
        // the self.words trie and keep it in step with built_word.
        let path_is_possible_word = match self.words.contains(&built_word[..]) {
            Presence::Missing => false,
            Presence::Prefix => true,
            Presence::Present => {
                found_words.insert(String::from_utf8(built_word.iter().map(|l| l + LETTER_OFFSET).collect()).unwrap());
                true
            },
        };

        if path_so_far.len() < max_word_len && path_is_possible_word {
            let (on_left_edge, on_right_edge) = {
                let mod_result = next_pos % board.width;
                (mod_result == 0, mod_result == board.width - 1)
            };
            let (on_top_edge, on_bottom_edge) = (
                next_pos < board.width,
                next_pos + board.width >= max_word_len
            );

            let mut try_including = |candidate_pos: usize| {
                if !path_so_far.contains(&candidate_pos) {
                    self.find_valid_words_inner(board, max_word_len, found_words, built_word, path_so_far, candidate_pos);
                }
            };

            // consider up to 8 surrounding positions
            if !on_top_edge {
                if !on_left_edge {
                    try_including(next_pos - board.width - 1);
                }
                try_including(next_pos - board.width);
                if !on_right_edge {
                    try_including(next_pos - board.width + 1);
                }
            }
            if !on_left_edge {
                try_including(next_pos - 1);
            }
            // (don't consider the 'middle' position, because we already include it)
            if !on_right_edge {
                try_including(next_pos + 1);
            }
            if !on_bottom_edge {
                if !on_left_edge {
                    try_including(next_pos + board.width - 1);
                }
                try_including(next_pos + board.width);
                if !on_right_edge {
                    try_including(next_pos + board.width + 1);
                }
            }
        }

        path_so_far.pop();
        let removed_char = built_word.pop().expect("built_word should not be empty");
        if removed_char == OFFSET_U && built_word.last() == Some(&OFFSET_Q) {
            // the U we removed was there because of a Q; so remove the Q too.
            built_word.pop();
        }
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
