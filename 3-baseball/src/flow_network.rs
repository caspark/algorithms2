use std::cell::Cell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

// based on http://algs4.cs.princeton.edu/64maxflow/FlowEdge.java.html
#[derive(Clone, PartialEq)]
pub struct FlowEdge {
    pub v: usize, // from edge
    pub w: usize, // to edge
    pub capacity: f64,
    flow: Cell<f64>,
}

impl FlowEdge {
    pub fn new(v: usize, w: usize, capacity: f64, flow: f64) -> FlowEdge {
        assert!(capacity >= 0.0, "edge capacity must be non-negative");
        assert!(flow <= capacity, "flow must not exceed capacity");
        assert!(flow >= 0.0, "flow must be non-negative");
        FlowEdge {
            v: v,
            w: w,
            capacity: capacity,
            flow: Cell::new(flow),
        }
    }

    pub fn flow(&self) -> f64 {
        self.flow.get()
    }

    pub fn other(&self, vertex: usize) -> usize {
        if vertex == self.v {
            self.w
        } else if vertex == self.w {
            self.v
        } else {
            panic!(format!("Vertex {} is not on this edge", vertex))
        }
    }

    pub fn residual_capacity_to(&self, vertex: usize) -> f64 {
        if vertex == self.v {
            self.flow() // backward edge
        } else if vertex == self.w {
            self.capacity - self.flow() // forward edge
        } else {
            panic!(format!("Vertex {} is not on this edge", vertex))
        }
    }

    /// Changes this edge to apply the given `delta` in flow to the given `vertex` (using the interior mutability provided by Cell)
    pub fn add_residual_flow_to(&self, vertex: usize, delta: f64) {
        if vertex == self.v {
            self.flow.set(self.flow.get() - delta); // backward edge
        } else if vertex == self.w {
            self.flow.set(self.flow.get() + delta); // forward edge
        } else {
            panic!(format!("Vertex {} is not on this edge", vertex));
        }
        assert!(!delta.is_nan(), "change in flow = NaN");
        assert!(self.flow() >= 0.0, "flow is negative");
        assert!(self.flow() <= self.capacity, "flow exceeds capacity");
    }
}

impl fmt::Debug for FlowEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {} ({}/{})", self.v, self.w, self.flow.get(), self.capacity)
    }
}

// based on http://algs4.cs.princeton.edu/64maxflow/FlowNetwork.java.html
pub struct FlowNetwork {
    adj: Vec<Vec<Rc<FlowEdge>>>,
}

impl FlowNetwork {
    pub fn new(v: usize) -> FlowNetwork {
        FlowNetwork {
            adj: (0..v).map(|_| Vec::new()).collect(),
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.adj.len()
    }

    fn validate_vertex(&self, vertex: usize) {
        assert!(vertex <= self.adj.len(), format!("vertex {} is not in the flow network!", vertex));
    }

    pub fn add_edge(&mut self, e: FlowEdge) {
        self.validate_vertex(e.v);
        self.validate_vertex(e.w);
        let rc_e = Rc::new(e);
        self.adj[rc_e.v].push(rc_e.clone());
        self.adj[rc_e.w].push(rc_e);
    }

    pub fn adj(&self, vertex: usize) -> Vec<&FlowEdge> {
        self.validate_vertex(vertex);
        // ideally we'd return an iterator instead, but Rust makes it very hard to do that at the moment
        let mut result = Vec::with_capacity(self.adj[vertex].len());
        for rc_e in self.adj[vertex].iter() {
            result.push(rc_e.deref());
        }
        result
    }
}

impl fmt::Debug for FlowNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Flow network with {} vertices:", self.adj.len()));
        for (vertex, edges) in self.adj.iter().map(|rc_e| rc_e.deref()).enumerate() {
            try!(write!(f, "\tVertex {} edges:", vertex));
            for edge in edges.iter() {
                try!(write!(f, "\t\t{:?}", edge));
            }
        }
        Ok(())
    }
}
