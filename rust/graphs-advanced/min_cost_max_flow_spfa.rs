// Min-Cost Max-Flow (classic)
// Send the maximum flow from s to t through a network where each edge has a capacity
// and per-unit cost, minimizing total cost among all maximum flows.
// Approach: successive shortest paths using SPFA (Bellman-Ford queue) on the residual
// graph, which tolerates the negative-cost reverse edges.
// Time: O(F * V * E) worst case, Space: O(V + E)
use std::collections::VecDeque;

struct Edge {
    to: usize,
    cap: i64,
    cost: i64,
}

pub struct Mcmf {
    edges: Vec<Edge>,
    g: Vec<Vec<usize>>,
}

impl Mcmf {
    pub fn new(n: usize) -> Self {
        Self { edges: vec![], g: vec![vec![]; n] }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost: i64) {
        self.g[u].push(self.edges.len());
        self.edges.push(Edge { to: v, cap, cost });
        self.g[v].push(self.edges.len());
        self.edges.push(Edge { to: u, cap: 0, cost: -cost });
    }

    /// Returns (max_flow, min_cost).
    pub fn run(&mut self, s: usize, t: usize) -> (i64, i64) {
        let n = self.g.len();
        let (mut flow, mut cost) = (0, 0);
        loop {
            let mut dist = vec![i64::MAX; n];
            let mut in_q = vec![false; n];
            let mut prev_edge = vec![usize::MAX; n];
            dist[s] = 0;
            let mut q = VecDeque::from([s]);
            while let Some(u) = q.pop_front() {
                in_q[u] = false;
                for &e in &self.g[u] {
                    let Edge { to, cap, cost: c } = self.edges[e];
                    if cap > 0 && dist[u] + c < dist[to] {
                        dist[to] = dist[u] + c;
                        prev_edge[to] = e;
                        if !in_q[to] {
                            in_q[to] = true;
                            q.push_back(to);
                        }
                    }
                }
            }
            if dist[t] == i64::MAX {
                return (flow, cost);
            }
            let mut push = i64::MAX;
            let mut v = t;
            while v != s {
                let e = prev_edge[v];
                push = push.min(self.edges[e].cap);
                v = self.edges[e ^ 1].to;
            }
            let mut v = t;
            while v != s {
                let e = prev_edge[v];
                self.edges[e].cap -= push;
                self.edges[e ^ 1].cap += push;
                v = self.edges[e ^ 1].to;
            }
            flow += push;
            cost += push * dist[t];
        }
    }
}

fn main() {
    let mut m = Mcmf::new(4);
    m.add_edge(0, 1, 2, 1);
    m.add_edge(0, 2, 1, 2);
    m.add_edge(1, 2, 1, 1);
    m.add_edge(1, 3, 1, 3);
    m.add_edge(2, 3, 2, 1);
    // paths: 0-1-2-3 cost 3, 0-2-3 cost 3, 0-1-3 cost 4 -> flow 3, cost 10
    assert_eq!(m.run(0, 3), (3, 10));
    println!("ok");
}
