use flow_network::{FlowEdge, FlowNetwork};
use std::collections::VecDeque;
use std::f64;

const EPSILON: f64 = 1e-11;

// Based on http://algs4.cs.princeton.edu/64maxflow/FordFulkerson.java.html
pub struct FordFulkersonResult<'graph> {
    marked: Vec<bool>, // marked[v] = true iff s->v path in residual graph
    edge_to: Vec<Option<&'graph FlowEdge>>, // edge_to[v] = last edge on shortest residual s->v path
    max_flow_value: f64,
}

impl<'graph> FordFulkersonResult<'graph> {
    fn has_augmenting_path(&mut self, g: &'graph FlowNetwork, s: usize, t: usize) -> bool {
        self.marked = vec![false; g.num_vertices()];
        self.edge_to = vec![None; g.num_vertices()];

        // breadth-first search
        let mut queue = VecDeque::new();
        queue.push_back(s);
        self.marked[s] = true;
        while !queue.is_empty() && !self.marked[t] {
            let v = queue.pop_front().expect("queue is known to be non-empty");
            for e in g.adj(v) {
                let w = e.other(v);
                if e.residual_capacity_to(w) > 0.0 {
                    if !self.marked[w] {
                        self.edge_to[w] = Some(e);
                        self.marked[w] = true;
                        queue.push_back(w);
                    }
                }
            }
        }

        self.marked[t]
    }

    pub fn in_cut(&self, vertex: usize) -> bool {
        debug_assert!(vertex < self.marked.len(), "Given vertex is outside the graph used to get this result");
        self.marked[vertex]
    }
}

/// Use the Ford-Fulkerson algorithm to find the min-cut/max-flow, updating flow in the graph as we go.
pub fn calculate(g: &FlowNetwork, s: usize, t: usize) -> FordFulkersonResult {
    assert!(s <= g.num_vertices(), "source vertex {} is not in flow network of size {}", s, g.num_vertices());
    assert!(t <= g.num_vertices(), "sink vertex {} is not in flow network of size {}", t, g.num_vertices());
    assert!(s != t, "source and target vertexes are the same! ({})", s);

    let mut result = FordFulkersonResult {
        marked: vec![false; g.num_vertices()],
        edge_to: vec![None; g.num_vertices()],
        max_flow_value: 0.0,
    };

    match infeasibility_check(g, s, t, result.max_flow_value) {
        Some(reason) => panic!("initial flow is infeasible: {}", reason),
        None => (),
    }

    result.max_flow_value = excess(g, t);
    while result.has_augmenting_path(g, s, t) {
        // compute bottleneck capacity
        let mut bottle = f64::INFINITY;
        {
            let mut v = t;
            while v != s {
                let edge_back = result.edge_to[v].expect("there is an augmenting path");
                bottle = if edge_back.residual_capacity_to(v) < bottle {
                    edge_back.residual_capacity_to(v)
                } else {
                    bottle
                };
                v = edge_back.other(v);
            }
        }

        { // augment flow
            let mut v = t;
            while v != s {
                let edge_back = result.edge_to[v].expect("there is an augmenting path");
                edge_back.add_residual_flow_to(v, bottle);
                v = edge_back.other(v);
            }
        }

        result.max_flow_value += bottle;
    }

    result
}

/// Returns the reason further flow is infeasible, or None if it's currently feasible
fn infeasibility_check(g: &FlowNetwork, s: usize, t: usize, max_flow_value: f64) -> Option<String> {
    for v in 0..g.num_vertices() {
        for e in g.adj(v) {
            if e.flow() < -EPSILON || e.flow() > e.capacity + EPSILON {
                return Some(format!("Edge does not satisfy capacity constraints: {:?}", e));
            }
        }
    }

    if (max_flow_value + excess(g, s)).abs() > EPSILON {
        return Some(format!("Excess at source = {}\nMax flow = {}", excess(g, s), max_flow_value));
    }

    if (max_flow_value - excess(g, t)).abs() > EPSILON {
        return Some(format!("Excess at sink = {}\nMax flow = {}", excess(g, t), max_flow_value));
    }

    for v in 0..g.num_vertices() {
        if v == s || v == t {
            continue
        } else if excess(g, v).abs() > EPSILON {
            return Some(format!("Net flow out of {} doesn't equal zero", v))
        }
    }

    None // there's no reason it's not feasible (aka it's feasible)
}

fn excess(g: &FlowNetwork, v: usize) -> f64 {
    let mut excess = 0.0f64;
    for e in g.adj(v) {
        if v == e.v {
            excess -= e.flow();
        } else {
            excess += e.flow();
        }
    }
    excess
}

