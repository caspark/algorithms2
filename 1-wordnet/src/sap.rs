use std::slice::Iter;
use digraph::Digraph;

struct SAP {
    graph: Digraph,
}

// functions to calculate the shortest ancestral path between vertices of a digraph
// a note on design: these functions might be better off as free-standing functions, but they're grouped in the Java
// API that's required so may as well group them here to match that.
impl SAP {
    pub fn path_length_between(&self, v: i32, w: i32) -> Option<i32> {

        //TODO should be able to calculate this by doing a BFS from v and w.
        // Then for each vertex:
        //   if the vertex is reachable from both v and w:
        //     add num steps to get there from v and w.
        //     if this total is the best we've seen so far, save the vertex as the new best common ancestor
        // Then return sum the lengths of the path from v -> common ancestor + w -> common ancestor
        panic!("Not yet implemented");
    }

    pub fn common_ancestor_of(&self, v: i32, w: i32) -> Option<i32> {
        panic!("Not yet implemented");
    }

    pub fn path_length_between_any_of(&self, vs: Iter<i32>, ws: Iter<i32>) -> Option<i32> {
        panic!("Not yet implemented");
    }

    pub fn common_ancestor_of_any_of(&self, vs: Iter<i32>, ws: Iter<i32>) -> Option<i32> {
        panic!("Not yet implemented");
    }
}

#[cfg(test)]
mod tests {
    use digraph::Digraph;
    use super::{SAP};

    #[test]
    fn should_find_shortest_path_in_trivial_case() {
        let mut g = Digraph::new(1);

        let sap = SAP { graph: g };
        assert_eq!(sap.path_length_between(0, 0), Some(0));
        assert_eq!(sap.common_ancestor_of(0, 0), Some(0));
    }

    #[test]
    fn should_find_shortest_path_in_triangle() {
        let mut g = Digraph::new(3);

        g.add_edge(0, 1);
        g.add_edge(2, 1);

        let sap = SAP { graph: g };
        assert_eq!(sap.path_length_between(0, 2), Some(2));
        assert_eq!(sap.common_ancestor_of(0, 2), Some(1));
    }

    //TODO add more tests
}
