#![feature(core)] // allows using sum() on iterators

use std::env;
use wordnet::WordNet;

mod bfdp;
mod digraph;
mod outcast;
mod sap;
mod wordnet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage: {} <synsets-file> <hypernyms-file> <outcast-file-1> [... <outcast-file-n>]", args[0]);
    if args.len() < 4 {
        println!("Error: incorrect number of arguments provided.\n{}", usage);
        return;
    }

    let (synsets_path, hypernyms_path) = (&args[1], &args[2]);
    match WordNet::create_by_parsing_files(synsets_path, hypernyms_path) {
        Ok(wordnet) => for outcast_path in args.iter().skip(3) {
            print!("{} ", outcast_path);
            match read_nouns(outcast_path) {
                Ok(nouns) => println!("outcast is {} (nouns: {:?})", outcast::find_outcast(&wordnet, &nouns), &nouns),
                Err(parse_err) => panic!("Cannot read nouns from {}; {}", outcast_path, parse_err),
            }
        },
        Err(parse_err) => panic!("Failed parsing synsets or hypernyms: {}", parse_err),
    }

    println!("Finished.");
}

fn read_nouns(path: &String) -> std::io::Result<Vec<String>> {
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::fs::File;

    let file = BufReader::new(try!(File::open(path)));
    Ok(file.lines().map(|s| s.unwrap()).filter(|s| s.len() > 0).collect())
}
