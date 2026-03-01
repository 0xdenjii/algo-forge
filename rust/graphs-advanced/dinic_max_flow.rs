// Maximum Flow - Dinic's Algorithm (classic)
// Given a directed graph with edge capacities, compute the maximum flow from s to t.
// Approach: repeatedly build a BFS level graph, then push blocking flow with DFS
// using per-vertex edge pointers (current-arc optimization). Residual edges are
// stored in pairs so `e ^ 1` is the reverse edge.
// Time: O(V^2 * E) general, O(E * sqrt(V)) on unit networks; Space: O(V + E)
use std::collections::VecDeque;

pub struct Dinic {
    to: Vec<usize>,
    cap: Vec<i64>,
    head: Vec<Vec<usize>>,
    level: Vec<i32>,
    it: Vec<usize>,
}

impl Dinic {
    pub fn new(n: usize) -> Self {
        Self { to: vec![], cap: vec![], head: vec![vec![]; n], level: vec![0; n], it: vec![0; n] }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, c: i64) {
        self.head[u].push(self.to.len());
        self.to.push(v);
        self.cap.push(c);
        self.head[v].push(self.to.len());
        self.to.push(u);
        self.cap.push(0);
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level.iter_mut().for_each(|l| *l = -1);
        self.level[s] = 0;
        let mut q = VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &e in &self.head[u] {
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
        while self.it[u] < self.head[u].len() {
            let e = self.head[u][self.it[u]];
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
            self.it.iter_mut().for_each(|x| *x = 0);
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
    // CLRS example network, max flow = 23
    let mut g = Dinic::new(6);
    for &(u, v, c) in &[(0, 1, 16), (0, 2, 13), (1, 2, 10), (2, 1, 4), (1, 3, 12), (3, 2, 9), (2, 4, 14), (4, 3, 7), (3, 5, 20), (4, 5, 4)] {
        g.add_edge(u, v, c);
    }
    assert_eq!(g.max_flow(0, 5), 23);
    println!("ok");
}
