use bfdp;
use digraph::Digraph;

/// Calculates the shortest ancestral path between vertices of a digraph.
/// Returns the common ancestor closest to both points and the distance between those two points
/// when taking the path that passes through the common ancestor.
pub fn path_stats_between(g: &Digraph, vs: Vec<usize>, ws: Vec<usize>) -> Option<(i32, usize)> {
    let vs_result = bfdp::search(g, vs);
    let ws_result = bfdp::search(g, ws);

    // For each vertex:
    //   if the vertex is reachable from both v and w:
    //     add num steps to get there from v and w.
    //     if this total is the best we've seen so far, save the vertex as the new best common ancestor
    // Then return sum the lengths of the path from v -> common ancestor + w -> common ancestor

    let mut best_paths = None;
    for target in 0..g.vertices() {
        let target_paths = (vs_result.path_to(target), ws_result.path_to(target));
        match target_paths {
            (Some(v_tp), Some(w_tp)) => {
                //HACK this code is ugly as sin but I've yet to convince the borrow checker that
                //     there exists a nicer yet still legal way to express the same thing.
                if best_paths.is_none() {
                    best_paths = Some((v_tp, w_tp));
                } else {
                    let (v_bp, w_bp) = best_paths.expect("previously checked");
                    if v_tp.len() + w_tp.len() < v_bp.len() + w_bp.len() {
                        best_paths = Some((v_tp, w_tp));
                    } else {
                        best_paths = Some((v_bp, w_bp));
                    }
                }
            },
            _ => (),
        }
    }

    best_paths.map(|(v_bp, w_bp)| {
        (
            (v_bp.len() + w_bp.len() - 2) as i32, // path length
            v_bp.last().expect("paths must have at least 1 element").clone() // common ancestor
        )
    })
}

#[cfg(test)]
mod tests {
    use digraph::Digraph;
    use super::*;

    #[test]
    fn should_find_shortest_path_in_trivial_case() {
        let g = Digraph::new(1);

        assert_eq!(path_stats_between(&g, vec![0], vec![0]), Some((0, 0usize)));
    }

    #[test]
    fn should_find_shortest_path_in_triangle() {
        // Graph: 0 -> 1 -> 2
        //         \-->----/
        let mut g = Digraph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);

        assert_eq!(path_stats_between(&g, vec![0], vec![2]), Some((1, 2usize)));
    }

    #[test]
    fn should_find_path_when_both_need_to_move() {
        // Graph: 0 ->  2
        //        1 ->-/
        let mut g = Digraph::new(3);
        g.add_edge(0, 2);
        g.add_edge(1, 2);

        assert_eq!(path_stats_between(&g, vec![0], vec![1]), Some((2, 2usize)));
    }

    //TODO add more tests for multiple search starting positions
}
