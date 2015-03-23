use std::slice::Iter;

struct Digraph {
    v: i32,
    e: i32,
    adj: Vec<Vec<i32>>,
}

impl Digraph {
    pub fn new(v: i32) -> Digraph {
        assert!(v >= 0, "number of vertices must be at least zero");
        Digraph {
            v: v,
            e: 0,
            adj: (0..v).map(|_| Vec::new()).collect::<Vec<_>>(),
        }
    }

    pub fn vertices(&self) -> i32 {
        self.v
    }

    pub fn edges(&self) -> i32 {
        self.e
    }

    fn checked_vertex(&self, vertex: i32) -> i32 {
        assert!(vertex >= 0, format!("Vertex {} must be >= 0", vertex));
        assert!(vertex < self.v, format!("Vertex {} must be < num vertexes ({})", vertex, self.v));
        vertex
    }

    pub fn add_edge(&mut self, v: i32, w: i32) {
        let from_index = self.checked_vertex(v) as usize;
        let checked_index = self.checked_vertex(w);
        self.adj[from_index].push(checked_index);
        self.e += 1;
    }

    fn adj(&self, vertex: i32) -> Iter<i32> {
        let checked_index = self.checked_vertex(vertex) as usize;
        self.adj[checked_index].iter()
    }
}
