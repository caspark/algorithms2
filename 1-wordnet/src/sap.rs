use bfdp;
use digraph::Digraph;

/// Calculates the shortest ancestral path between vertices of a digraph.
/// Returns the distance - number of edges between those two points when taking the path that passes through the common
/// ancestor.
pub fn path_stats_between(g: &Digraph, vs: Vec<usize>, ws: Vec<usize>) -> Option<i32> {
    let vs_result = bfdp::search(g, vs);
    let ws_result = bfdp::search(g, ws);

    // For each vertex:
    //   if the vertex is reachable from both v and w:
    //     add num steps to get there from v and w.
    //     if this total is the best we've seen so far, save the vertex as the new best common ancestor
    // Then return sum the lengths of the path from v -> common ancestor + w -> common ancestor

    let mut best_path = None;
    for target in 0..g.vertices() {
        let target_paths = (vs_result.dist_to(target), ws_result.dist_to(target));
        match target_paths {
            (Some(v_tp), Some(w_tp)) => {
                best_path = Some(match best_path {
                    None => v_tp + w_tp,
                    Some(best_path_len) =>
                        if v_tp + w_tp < best_path_len {
                            v_tp + w_tp
                        } else {
                            best_path_len
                        },
                });
            },
            _ => (),
        }
    }

    best_path.map(|bp| bp as i32)
}

#[cfg(test)]
mod tests {
    use digraph::Digraph;
    use super::*;

    #[test]
    fn should_find_shortest_path_in_trivial_case() {
        let g = Digraph::new(1);

        assert_eq!(path_stats_between(&g, vec![0], vec![0]), Some(0));
    }

    #[test]
    fn should_find_shortest_path_in_triangle() {
        // Graph: 0 -> 1 -> 2
        //         \-->----/
        let mut g = Digraph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);

        assert_eq!(path_stats_between(&g, vec![0], vec![2]), Some(1));
    }

    #[test]
    fn should_find_path_when_both_need_to_move() {
        // Graph: 0 ->  2
        //        1 ->-/
        let mut g = Digraph::new(3);
        g.add_edge(0, 2);
        g.add_edge(1, 2);

        assert_eq!(path_stats_between(&g, vec![0], vec![1]), Some(2));
    }

    #[test]
    fn should_find_path_when_having_multiple_sources_and_dests() {
        // Graph: 0 -> 1 ->  2 <- 4 <- 5  <- 6
        //             3 ->-/           \-<- 7 <- 8
        let mut g = Digraph::new(9);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(3, 2);
        g.add_edge(4, 2);
        g.add_edge(5, 4);
        g.add_edge(6, 5);
        g.add_edge(7, 5);
        g.add_edge(8, 7);

        assert_eq!(path_stats_between(&g, vec![0, 3], vec![6, 8]), Some(4));
    }
}
