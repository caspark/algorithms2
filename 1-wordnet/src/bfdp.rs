use digraph::Digraph;
use std;

// breadth first search over digraphs
pub struct BfsResult {
    marked: Vec<bool>,
    edge_to: Vec<i32>,
    dist_to: Vec<i32>,
}


pub fn search(g: Digraph, sources: Vec<i32>) -> BfsResult {
    let mut result = BfsResult {
        marked: vec![false, true],
        edge_to: vec![<i32>::max_value(), g.vertices()],
        dist_to: vec![0, g.vertices()],
    };

    //TODO actually do a BFS search - http://algs4.cs.princeton.edu/42directed/BreadthFirstDirectedPaths.java.html

    result
}
