// Title: Lowest Common Ancestor via Binary Lifting
// Problem: Rooted tree with n nodes; answer queries LCA(u, v) and dist(u, v) (e.g. CSES
// Company Queries II, Distance Queries).
// Approach: BFS for depth and parent; up[k][v] = 2^k-th ancestor. Lift the deeper node to
// equal depth, then lift both while ancestors differ.
// Complexity: O(n log n) preprocessing and space, O(log n) per query.
use std::collections::VecDeque;

pub struct Lca {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl Lca {
    pub fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len();
        let lg = (usize::BITS - n.leading_zeros()).max(1) as usize;
        let mut up = vec![vec![root; n]; lg];
        let mut depth = vec![0; n];
        let mut seen = vec![false; n];
        let mut q = VecDeque::from([root]);
        seen[root] = true;
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    depth[v] = depth[u] + 1;
                    up[0][v] = u;
                    q.push_back(v);
                }
            }
        }
        for k in 1..lg {
            for v in 0..n {
                up[k][v] = up[k - 1][up[k - 1][v]];
            }
        }
        Self { up, depth }
    }

    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        let diff = self.depth[a] - self.depth[b];
        for k in 0..self.up.len() {
            if (diff >> k) & 1 == 1 {
                a = self.up[k][a];
            }
        }
        if a == b {
            return a;
        }
        for k in (0..self.up.len()).rev() {
            if self.up[k][a] != self.up[k][b] {
                a = self.up[k][a];
                b = self.up[k][b];
            }
        }
        self.up[0][a]
    }

    pub fn dist(&self, a: usize, b: usize) -> usize {
        self.depth[a] + self.depth[b] - 2 * self.depth[self.lca(a, b)]
    }
}

fn main() {
    let edges = [(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (4, 6)];
    let mut adj = vec![Vec::new(); 7];
    for &(a, b) in &edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    let l = Lca::new(&adj, 0);
    assert_eq!(l.lca(3, 6), 1);
    assert_eq!(l.lca(6, 5), 0);
    assert_eq!(l.lca(4, 6), 4);
    assert_eq!(l.dist(3, 5), 4);
    println!("ok");
}
