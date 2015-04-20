use digraph::Digraph;
use std::collections::VecDeque;

// breadth first search over digraphs
pub struct BfsResult {
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    dist_to: Vec<i32>,
}

pub fn search(g: &Digraph, sources: Vec<usize>) -> BfsResult {
    let num_vertices = g.vertices();
    let mut result = BfsResult {
        marked: vec![false; num_vertices],
        edge_to: vec![<usize>::max_value(); num_vertices],
        dist_to: vec![0; num_vertices],
    };

    let mut q = VecDeque::new();
    for v in sources {
        result.marked[v] = true;
        result.dist_to[v] = 0;
        q.push_back(v);
    }
    while q.len() > 0 {
        let v = q.pop_front().expect("queue length known to be > 0");
        for w in g.adj(v) {
            if !result.marked[*w] {
                result.edge_to[*w] = v;
                result.dist_to[*w] = result.dist_to[v] + 1;
                result.marked[*w] = true;
                q.push_back(*w);
            }
        }
    }

    result
}

impl BfsResult {
    /// Returns None if there is no path
    pub fn dist_to(&self, v: usize) -> Option<i32> {
        if self.marked[v] {
            Some(self.dist_to[v])
        } else {
            None
        }
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if self.marked[v] {
            let mut path = Vec::new();
            let mut x = v;
            while self.dist_to[x] != 0 {
                path.push(x);
                x = self.edge_to[x];
            }
            path.push(x);
            path.reverse();
            Some(path)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use digraph::Digraph;

    #[test]
    fn should_work_for_simple_case() {
        // Graph:
        // 0 -> 1 -> 2 -> 3
        // \-->-----/
        let mut g = Digraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(0, 2);

        let result = search(&g, vec![0]);

        assert_eq!(result.path_to(0), Some(vec![0]));
        assert_eq!(result.path_to(1), Some(vec![0, 1]));
        assert_eq!(result.path_to(2), Some(vec![0, 2]));
        assert_eq!(result.path_to(3), Some(vec![0, 2, 3]));

        assert_eq!(result.dist_to(0), Some(0));
        assert_eq!(result.dist_to(1), Some(1));
        assert_eq!(result.dist_to(2), Some(1));
        assert_eq!(result.dist_to(3), Some(2));
    }

    #[test]
    fn should_handle_disconnected_components() {
        // Graph: 0 -> 1     2     3
        let mut g = Digraph::new(4);
        g.add_edge(0, 1);

        let result = search(&g, vec![0, 2]);

        assert_eq!(result.path_to(0), Some(vec![0]));
        assert_eq!(result.path_to(1), Some(vec![0, 1]));
        assert_eq!(result.path_to(2), Some(vec![2]));
        assert_eq!(result.path_to(3), None);

        assert_eq!(result.dist_to(0), Some(0));
        assert_eq!(result.dist_to(1), Some(1));
        assert_eq!(result.dist_to(2), Some(0));
        assert_eq!(result.dist_to(3), None);
    }
}
