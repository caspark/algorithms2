#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

#[macro_use]
extern crate mdo;

mod digraph;
mod wordnet;


fn main() {
    println!("Hello, world!");
}
