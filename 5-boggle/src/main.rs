use std::env;
use std::process;
use std::io;
use boggle::{BoggleBoard, BoggleSolver};

mod boggle;
mod trie;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let print_usage = || {
        println!("Usage: {} DICTIONARY-FILE BOARD-FILE", program);
    };

    if args.len() != 3 {
        print_usage();
        process::exit(1);
    }

    let dict = parse_dictionary(&args[1]).unwrap();
    let solver = BoggleSolver::new(dict);

    let board = parse_boggle_board(&args[2]).unwrap();
    let words = solver.find_valid_words(&board);

    let mut score = 0;
    for word in words {
        println!("{}", word);
        score += BoggleSolver::score_of_word(&word);
    }
    println!("Score = {}", score);
}

fn parse_dictionary(dict_path: &String) -> io::Result<Vec<String>> {
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;

    print!("Parsing dictionary words from {} ...", dict_path);
    let dict_file = BufReader::new(try!(File::open(dict_path)));
    let mut words = Vec::with_capacity(500000);
    for line_or_err in dict_file.lines() {
        let word_line = line_or_err.unwrap();

        if word_line.len() == 0 {
            break; // end of file
        }
        words.push(word_line);
    }
    println!("done!");
    Ok(words)
}

fn parse_boggle_board(board_path: &String) -> io::Result<BoggleBoard> {
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;

    print!("Parsing board from {} ...", board_path);
    let board_file = BufReader::new(try!(File::open(board_path)));
    let mut width = None;
    let mut height = None;
    let mut letters = Vec::with_capacity(10000);
    for line_or_err in board_file.lines() {
        let line = line_or_err.unwrap();

        if line.len() == 0 {
            break; // end of file
        }

        for s in line.split(' ').filter(|s| s.len() != 0) {
            if width.is_none() {
                width = Some(s.parse().unwrap());
            } else if height.is_none() {
                height = Some(s.parse().unwrap());
            } else {
                letters.push(s.bytes().nth(0).expect("board letters must have first char"));
            }
        }
    }
    println!("done!");
    Ok(BoggleBoard::new(width.expect("board width must be specified"),
                        height.expect("board height must be specified"),
                        letters))
}

