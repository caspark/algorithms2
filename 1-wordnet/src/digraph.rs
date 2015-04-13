use std::slice::Iter;

pub struct Digraph {
    v: usize,
    e: i32,
    adj: Vec<Vec<usize>>,
}

impl Digraph {
    pub fn new(v: usize) -> Digraph {
        assert!(v >= 0, "number of vertices must be at least zero");
        Digraph {
            v: v,
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    pub fn vertices(&self) -> usize {
        self.v
    }

    pub fn edges(&self) -> i32 {
        self.e
    }

    fn checked_vertex(&self, vertex: usize) -> usize {
        assert!(vertex >= 0, format!("Vertex {} must be >= 0", vertex));
        assert!(vertex < self.v, format!("Vertex {} must be < num vertexes ({})", vertex, self.v));
        vertex
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        let from_index = self.checked_vertex(v);
        let checked_index = self.checked_vertex(w);
        self.adj[from_index].push(checked_index);
        self.e += 1;
    }

    pub fn adj(&self, vertex: usize) -> Iter<usize> {
        let checked_index = self.checked_vertex(vertex);
        self.adj[checked_index].iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_edges_should_make_src_vertex_adjacent_to_dest_vertex() {
        let mut g = Digraph::new(2);

        g.add_edge(0, 1);

        assert_eq!(g.adj(0).cloned().collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn adding_edge_should_not_make_dest_vertex_adjacent_to_src_vertex() {
        let mut g = Digraph::new(2);

        g.add_edge(0, 1);

        assert_eq!(g.adj(1).collect::<Vec<_>>().len(), 0);
    }
}
