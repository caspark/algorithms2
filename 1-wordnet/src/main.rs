#![feature(core)] // allows using sum() on iterators
#![feature(plugin)]

#![plugin(regex_macros)]
extern crate regex;

#[macro_use]
extern crate mdo;

mod bfdp;
mod digraph;
mod outcast;
mod sap;
mod wordnet;


fn main() {
    println!("Hello, world!");
}
