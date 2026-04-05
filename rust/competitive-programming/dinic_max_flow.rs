// Title: Maximum Flow (Dinic's Algorithm)
// Problem: Directed graph with edge capacities; compute the maximum flow from s to t
// (CSES Download Speed). With unit capacities it also solves bipartite matching.
// Approach: Build BFS level graphs; push blocking flow with DFS using current-arc pointers.
// Residual edges are stored in pairs so edge e's reverse is e ^ 1.
// Complexity: O(V^2 E) in general, O(E sqrt V) on unit networks; O(V + E) space.
use std::collections::VecDeque;

pub struct Dinic {
    g: Vec<Vec<usize>>,
    to: Vec<usize>,
    cap: Vec<i64>,
    level: Vec<i32>,
    it: Vec<usize>,
}

impl Dinic {
    pub fn new(n: usize) -> Self {
        Self { g: vec![Vec::new(); n], to: vec![], cap: vec![], level: vec![0; n], it: vec![0; n] }
    }
    pub fn add_edge(&mut self, u: usize, v: usize, c: i64) {
        self.g[u].push(self.to.len());
        self.to.push(v);
        self.cap.push(c);
        self.g[v].push(self.to.len());
        self.to.push(u);
        self.cap.push(0);
    }
    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level.fill(-1);
        self.level[s] = 0;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &e in &self.g[u] {
                let v = self.to[e];
                if self.cap[e] > 0 && self.level[v] < 0 {
                    self.level[v] = self.level[u] + 1;
                    q.push_back(v);
                }
            }
        }
        self.level[t] >= 0
    }
    fn dfs(&mut self, u: usize, t: usize, f: i64) -> i64 {
        if u == t {
            return f;
        }
        while self.it[u] < self.g[u].len() {
            let e = self.g[u][self.it[u]];
            let v = self.to[e];
            if self.cap[e] > 0 && self.level[v] == self.level[u] + 1 {
                let d = self.dfs(v, t, f.min(self.cap[e]));
                if d > 0 {
                    self.cap[e] -= d;
                    self.cap[e ^ 1] += d;
                    return d;
                }
            }
            self.it[u] += 1;
        }
        0
    }
    pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        while self.bfs(s, t) {
            self.it.fill(0);
            loop {
                let f = self.dfs(s, t, i64::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
}

fn main() {
    // CLRS example network: max flow 23.
    let mut d = Dinic::new(6);
    let edges = [
        (0, 1, 16), (0, 2, 13), (1, 2, 10), (2, 1, 4), (1, 3, 12),
        (3, 2, 9), (2, 4, 14), (4, 3, 7), (3, 5, 20), (4, 5, 4),
    ];
    for &(u, v, c) in &edges {
        d.add_edge(u, v, c);
    }
    assert_eq!(d.max_flow(0, 5), 23);
    println!("ok");
}
